use std::{
    fs::File,
    io::{self, Seek, Write},
    path::Path,
};

use anyhow::{bail, Result};
use fxhash::FxHashMap;
use logos::Span;
use md5::{Digest, Md5};
use serde_json::json;
use smol_str::SmolStr;
use zip::{
    write::{FileOptions, ZipWriter},
    CompressionMethod,
};

use self::{
    node::Node,
    node_id::{NodeID, NodeIDFactory},
};
use crate::{
    ast::{
        Costume, Event, EventDetail, Expr, OnMessage, Proc, Project, Sprite, Stmt,
        Stmts,
    },
    blocks::{BinOp, Block, UnOp},
    config::Config,
    diagnostic::{keys::is_key, Diagnostic, DiagnosticKind},
};

pub mod node;
pub mod node_id;

pub struct Sb3<T>
where T: Write + Seek
{
    zip: ZipWriter<T>,
    id: NodeIDFactory,
    costumes: FxHashMap<SmolStr, SmolStr>,
    blocks_comma: bool,
    inputs_comma: bool,
}

type D<'a> = &'a mut Vec<Diagnostic>;

#[derive(Copy, Clone)]
struct S<'a> {
    stage: Option<&'a Sprite>,
    sprite: &'a Sprite,
    proc: Option<&'a Proc>,
}

impl<'a> S<'a> {
    fn is_arg(self, name: &str) -> bool {
        self.proc.is_some_and(|it| it.used_args.contains_key(name))
    }

    fn is_local_var(self, name: &str) -> bool {
        self.proc.is_some_and(|it| it.locals.contains_key(name))
    }

    fn is_var(self, name: &str) -> bool {
        self.sprite.vars.contains_key(name)
            || self.stage.is_some_and(|it| it.vars.contains_key(name))
    }

    fn is_list(self, name: &str) -> bool {
        self.sprite.lists.contains_key(name)
            || self.stage.is_some_and(|it| it.lists.contains_key(name))
    }
}

impl Stmt {
    fn is_terminator(&self) -> bool {
        matches!(
            self,
            Stmt::Forever { .. }
                | Stmt::Block {
                    block: Block::DeleteThisClone
                        | Block::StopAll
                        | Block::StopThisScript,
                    ..
                }
        )
    }

    fn opcode(&self, s: S) -> &'static str {
        match self {
            Stmt::Repeat { .. } => "control_repeat",
            Stmt::Forever { .. } => "control_forever",
            Stmt::Branch { else_body, .. } => {
                if else_body.is_empty() {
                    "control_if"
                } else {
                    "control_if_else"
                }
            }
            Stmt::Until { .. } => "control_repeat_until",
            Stmt::SetVar { .. } => "data_setvariableto",
            Stmt::ChangeVar { .. } => "data_changevariableby",
            Stmt::Show { name, .. } | Stmt::Hide { name, .. } => {
                if s.is_var(name) || s.is_local_var(name) {
                    "data_showvariable"
                } else {
                    "data_showlist"
                }
            }
            Stmt::ListAdd { .. } => "data_addtolist",
            Stmt::ListDelete { .. } => "data_deleteoflist",
            Stmt::ListDeleteAll { .. } => "data_deletealloflist",
            Stmt::ListInsert { .. } => "data_insertatlist",
            Stmt::ListSet { .. } | Stmt::ListChange { .. } => "data_replaceitemoflist",
            Stmt::Block { block, .. } => block.opcode(),
            Stmt::ProcCall { .. } => "procedures_call",
        }
    }
}

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn new(file: T) -> Self {
        Self {
            zip: ZipWriter::new(file),
            id: Default::default(),
            costumes: Default::default(),
            blocks_comma: false,
            inputs_comma: false,
        }
    }

    pub fn package(
        &mut self,
        project: &Project,
        config: &Config,
        input: &Path,
        stage_diags: D,
        diags: &mut FxHashMap<SmolStr, Vec<Diagnostic>>,
    ) -> Result<()> {
        self.zip.start_file(
            "project.json",
            FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .compression_level(Some(6)),
        )?;
        self.write_all(br#"{"targets":["#)?;
        self.sprite(None, &project.stage, stage_diags, "Stage", config, input)?;
        for (name, sprite) in project.sprites.iter() {
            self.write_all(b",")?;
            self.sprite(
                Some(&project.stage),
                sprite,
                diags.get_mut(name).unwrap(),
                name.as_str(),
                config,
                input,
            )?;
        }
        self.write_all(br#"],"monitors":[],"extensions":[],"meta":{"semver":"3.0.0","vm":"0.2.0","agent":"goboscript"}}"#)?;
        self.assets(input)?;
        self.zip.finish()?;
        Ok(())
    }

    fn assets(&mut self, input: &Path) -> Result<()> {
        for (path, hash) in &self.costumes {
            let (_, extension) = path.rsplit_once('.').unwrap();
            self.zip
                .start_file(format!("{hash}.{extension}"), FileOptions::default())?;
            let file = File::open(input.join(path.as_str()));
            io::copy(&mut file?, &mut self.zip)?;
        }
        Ok(())
    }

    fn sprite(
        &mut self,
        stage: Option<&Sprite>,
        sprite: &Sprite,
        diags: D,
        name: &str,
        config: &Config,
        input: &Path,
    ) -> Result<()> {
        self.id.reset();
        if name == "Stage" {
            self.write_all(br#"{"isStage":true"#)?;
            if !config.is_default() {
                write!(
                    self,
                    r#","comments":{{"a":{{"blockId":null,"x":0,"y":0,"width":350,"height":170,"minimized":false,"text":{}}}}}"#,
                    json!(config.to_string())
                )?;
            }
        } else {
            self.write_all(br#"{"isStage":false"#)?;
        }
        write!(self, r#","name":{},"blocks":{{"#, json!(name))?;
        self.blocks_comma = false;
        for proc in sprite.procs.values() {
            for (name, is_used) in &proc.used_args {
                let span =
                    proc.args.iter().find(|(arg, _)| arg == name).unwrap().1.clone();
                if !is_used {
                    diags.push(
                        DiagnosticKind::UnusedArgument(name.clone())
                            .to_diagnostic(span),
                    );
                }
            }
            if !sprite.used_procs.contains(&proc.name) {
                diags.push(
                    DiagnosticKind::UnusedProcedure(proc.name.clone())
                        .to_diagnostic(proc.span.clone()),
                );
            }
            self.proc(S { stage, sprite, proc: Some(proc) }, diags, proc)?;
        }
        for event in &sprite.events {
            self.event(S { stage, sprite, proc: None }, diags, event)?;
        }
        for on_msg in &sprite.on_messages {
            self.on_message(S { stage, sprite, proc: None }, diags, on_msg.1)?;
        }

        self.write_all(br#"},"costumes":["#)?;
        let mut comma = false;
        for costume in &sprite.costumes {
            self.comma(&mut comma)?;
            self.costume(diags, costume, input)?;
        }
        self.write_all(br#"],"variables":{"#)?;
        let mut comma = false;
        for proc in sprite.procs.values() {
            for var in proc.locals.values() {
                let resolved = json!(local_variable_resolved_name(proc, &var.name));
                self.comma(&mut comma)?;
                write!(self, r#"{}:[{},{}]"#, resolved, resolved, json!(var.default))?;
            }
        }
        for var in sprite.vars.values() {
            if !var.used {
                diags.push(
                    DiagnosticKind::UnusedVariable(var.name.clone())
                        .to_diagnostic(var.span.clone()),
                );
            }
            self.comma(&mut comma)?;
            write!(
                self,
                r#"{}:[{},{}]"#,
                json!(*var.name),
                json!(*var.name),
                json!(var.default)
            )?;
        }
        self.write_all(br#"},"lists":{"#)?;
        let mut comma = false;
        for list in sprite.lists.values() {
            if !list.used {
                diags.push(
                    DiagnosticKind::UnusedList(list.name.clone())
                        .to_diagnostic(list.span.clone()),
                );
            }
            self.comma(&mut comma)?;
            write!(
                self,
                r#"{}:[{},{}]"#,
                json!(*list.name),
                json!(*list.name),
                json!(list.default)
            )?;
        }
        self.write_all(br#"},"broadcasts":{"#)?;
        let mut comma = false;
        for broadcast_name in sprite.broadcasts.iter() {
            self.comma(&mut comma)?;
            write!(self, r#"{}:{}"#, json!(**broadcast_name), json!(**broadcast_name),)?;
        }
        // FIXME: Can you please fucking implement sounds this time?
        self.write_all(br#"},"sounds":[]}"#)?;
        for enum_ in sprite.enums.values() {
            for (variant, span) in &enum_.variants {
                if !enum_.used_variants.contains(variant) {
                    diags.push(
                        DiagnosticKind::UnusedEnumVariant {
                            enum_name: enum_.name.clone(),
                            variant_name: variant.clone(),
                        }
                        .to_diagnostic(span.clone()),
                    )
                }
            }
        }
        if sprite.costumes.is_empty() {
            diags.push(DiagnosticKind::NoCostumes.to_diagnostic(0..0))
        }
        Ok(())
    }

    fn costume(&mut self, d: D, costume: &Costume, input: &Path) -> Result<()> {
        if let Some(hash) = self.costumes.get(&costume.path) {
            let (_, extension) = costume.path.rsplit_once('.').unwrap();
            write!(
                self.zip,
                r#"{{"name":{},"assetId":"{hash}","dataFormat":"{extension}","md5ext":"{hash}.{extension}"}}"#,
                json!(*costume.name),
            )?;
            return Ok(());
        }
        let path = input.join(costume.path.as_str());
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                if matches!(err.kind(), io::ErrorKind::NotFound) {
                    d.push(
                        DiagnosticKind::FileNotFound(costume.path.clone())
                            .to_diagnostic(costume.span.clone()),
                    );
                    return Ok(());
                }
                bail!(err);
            }
        };
        let mut hasher = Md5::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = format!("{:x}", hasher.finalize());
        self.costumes.insert(costume.path.clone(), hash.into());
        self.costume(d, costume, input)
    }

    fn proc(&mut self, s: S, d: D, proc: &Proc) -> Result<()> {
        let this_id = self.id.new_id();
        let prototype_id = self.id.new_id();
        let next_id = self.id.new_id();
        self.node(
            Node::new("procedures_definition", this_id)
                .some_next_id((!proc.body.is_empty()).then_some(next_id))
                .top_level(true),
        )?;
        self.inputs()?;
        write!(self, r#""custom_block":[1,{prototype_id}]"#)?;
        self.end_obj()?;
        self.end_obj()?;

        let mut arg_ids = Vec::with_capacity(proc.args.len());
        for (arg, _) in &proc.args {
            let arg_id = self.id.new_id();
            arg_ids.push(arg_id);
            self.node(
                Node::new("argument_reporter_string_number", arg_id)
                    .parent_id(prototype_id)
                    .shadow(true),
            )?;
            self.single_field("VALUE", arg)?;
            self.end_obj()?;
        }

        self.node(
            Node::new("procedures_prototype", prototype_id)
                .parent_id(this_id)
                .shadow(true),
        )?;
        self.inputs()?;
        let mut comma = false;
        for ((arg, _), arg_id) in proc.args.iter().zip(arg_ids) {
            self.comma(&mut comma)?;
            write!(self, r#"{}:[2,{arg_id}]"#, json!(**arg))?;
        }
        self.end_obj()?;
        self.mutation()?;
        self.proccode(&proc.name, proc.args.len())?;
        self.argument_array("argumentids", &proc.args)?;
        self.argument_array("argumentnames", &proc.args)?;
        self.write_all(br#","argumentdefaults":"["#)?;
        let mut comma = false;
        for _ in &proc.args {
            self.comma(&mut comma)?;
            self.write_all(br#"\"\""#)?;
        }
        self.write_all(br#"]""#)?;
        self.warp(proc.warp)?;
        self.end_obj()?;
        self.end_obj()?;
        self.stmts(s, d, &proc.body, next_id, Some(this_id))
    }

    fn on_message(&mut self, s: S, d: D, on_message: &OnMessage) -> Result<()> {
        let this_id = self.id.new_id();
        let next_id = self.id.new_id();
        self.node(
            Node::new("event_whenbroadcastreceived", this_id)
                .some_next_id((!on_message.body.is_empty()).then_some(next_id))
                .top_level(true),
        )?;

        write!(
            self,
            r#","fields":{{"BROADCAST_OPTION":[{},{}]}}}}"#,
            json!(*on_message.message),
            json!(*on_message.message)
        )?;
        self.stmts(s, d, &on_message.body, next_id, Some(this_id))
    }

    fn event(&mut self, s: S, d: D, event: &Event) -> Result<()> {
        let this_id = self.id.new_id();
        let next_id = self.id.new_id();
        self.node(
            Node::new(event.opcode(), this_id)
                .some_next_id((!event.body.is_empty()).then_some(next_id))
                .top_level(true),
        )?;
        match &event.kind {
            EventDetail::OnKey { key, span } => {
                if !is_key(key) {
                    d.push(
                        DiagnosticKind::UnrecognizedKey(key.clone())
                            .to_diagnostic(span.clone()),
                    );
                }
                write!(
                    self,
                    r#","fields":{{"KEY_OPTION":[{},null]}}}}"#,
                    json!(**key)
                )?;
            }
            EventDetail::OnBackdrop { backdrop, span: _ } => {
                write!(
                    self,
                    r#","fields":{{"BACKDROP_OPTION":[{},null]}}}}"#,
                    json!(**backdrop)
                )?;
            }
            EventDetail::OnLoudnessGt { value } | EventDetail::OnTimerGt { value } => {
                let value_id = self.id.new_id();
                self.inputs()?;
                self.input(s, d, "VALUE", &value.borrow(), value_id)?;
                self.end_obj()?;
                self.write_all(
                    if matches!(event.kind, EventDetail::OnLoudnessGt { .. }) {
                        br#","fields":{"WHENGREATERTHANMENU":["LOUDNESS",null]}}"#
                    } else {
                        br#","fields":{"WHENGREATERTHANMENU":["TIMER",null]}}"#
                    },
                )?;
                self.expr(s, d, &value.borrow(), value_id, this_id)?;
            }
            _ => {
                self.end_obj()?;
            }
        }
        self.stmts(s, d, &event.body, next_id, Some(this_id))
    }

    fn stmts(
        &mut self,
        s: S,
        d: D,
        stmts: &Stmts,
        mut this_id: NodeID,
        mut parent_id: Option<NodeID>,
    ) -> Result<()> {
        for (i, stmt) in stmts.iter().enumerate() {
            let is_last = i == stmts.len() - 1;
            if is_last || stmt.is_terminator() {
                self.stmt(s, d, stmt, this_id, None, parent_id)?;
                if !is_last {
                    d.push(
                        DiagnosticKind::FollowedByUnreachableCode
                            .to_diagnostic(stmt.span().clone()),
                    )
                }
                break;
            }
            let next_id = self.id.new_id();
            self.stmt(s, d, stmt, this_id, Some(next_id), parent_id)?;
            parent_id = Some(this_id);
            this_id = next_id;
        }
        Ok(())
    }

    fn stmt(
        &mut self,
        s: S,
        d: D,
        stmt: &Stmt,
        this_id: NodeID,
        next_id: Option<NodeID>,
        parent_id: Option<NodeID>,
    ) -> Result<()> {
        self.node(
            Node::new(stmt.opcode(s), this_id)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.inputs()?;
        match stmt {
            Stmt::Forever { body, .. } => {
                let body_id = self.id.new_id();
                self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
                self.end_obj()?;
                self.end_obj()?;
                self.stmts(s, d, body, body_id, Some(this_id))?;
            }
            Stmt::Branch { cond, if_body, else_body } => {
                let cond_id = self.id.new_id();
                let if_body_id = self.id.new_id();
                let else_body_id = self.id.new_id();
                self.input(s, d, "CONDITION", &cond.borrow(), cond_id)?;
                self.substack("SUBSTACK", (!if_body.is_empty()).then_some(if_body_id))?;
                self.substack(
                    "SUBSTACK2",
                    (!else_body.is_empty()).then_some(else_body_id),
                )?;
                self.end_obj()?;
                self.end_obj()?;
                self.expr(s, d, &cond.borrow(), cond_id, this_id)?;
                self.stmts(s, d, if_body, if_body_id, Some(this_id))?;
                self.stmts(s, d, else_body, else_body_id, Some(this_id))?;
            }
            Stmt::Repeat { times: input, body } | Stmt::Until { cond: input, body } => {
                let input_id = self.id.new_id();
                let body_id = self.id.new_id();
                self.input(
                    s,
                    d,
                    if matches!(stmt, Stmt::Until { .. }) {
                        "CONDITION"
                    } else {
                        "TIMES"
                    },
                    &input.borrow(),
                    input_id,
                )?;
                self.substack("SUBSTACK", (!body.is_empty()).then_some(body_id))?;
                self.end_obj()?;
                self.end_obj()?;
                self.expr(s, d, &input.borrow(), input_id, this_id)?;
                self.stmts(s, d, body, body_id, Some(this_id))?;
            }
            | Stmt::SetVar { name, span, value, .. }
            | Stmt::ChangeVar { name, span, value } => {
                let value_id = self.id.new_id();
                self.input(s, d, "VALUE", &value.borrow(), value_id)?;
                self.end_obj()?;
                self.resolve_variable(s, d, name, span)?;
                self.end_obj()?;
                self.expr(s, d, &value.borrow(), value_id, this_id)?;
            }
            Stmt::Show { name, span } | Stmt::Hide { name, span } => {
                self.end_obj()?;
                self.resolve_variable_or_list(s, d, name, span)?;
                self.end_obj()?;
            }
            | Stmt::ListAdd { name, span, value: input }
            | Stmt::ListDelete { name, span, index: input } => {
                let input_id = self.id.new_id();
                self.list(s, d, name, span);
                self.input(
                    s,
                    d,
                    if matches!(stmt, Stmt::ListAdd { .. }) { "ITEM" } else { "INDEX" },
                    &input.borrow(),
                    input_id,
                )?;
                self.end_obj()?;
                self.single_field_id("LIST", name)?;
                self.end_obj()?;
                self.expr(s, d, &input.borrow(), input_id, this_id)?;
            }
            Stmt::ListDeleteAll { name, span } => {
                self.list(s, d, name, span);
                self.end_obj()?;
                self.single_field_id("LIST", name)?;
                self.end_obj()?;
            }
            | Stmt::ListInsert { name, span, index, value }
            | Stmt::ListSet { name, span, index, value } => {
                let index_id = self.id.new_id();
                let value_id = self.id.new_id();
                self.list(s, d, name, span);
                self.input(s, d, "INDEX", &index.borrow(), index_id)?;
                self.input(s, d, "ITEM", &value.borrow(), value_id)?;
                self.end_obj()?;
                self.single_field_id("LIST", name)?;
                self.end_obj()?;
                self.expr(s, d, &index.borrow(), index_id, this_id)?;
                self.expr(s, d, &value.borrow(), value_id, this_id)?;
            }
            Stmt::ListChange { index, name, op, span, value } => {
                let index_id = self.id.new_id();
                let value_id = self.id.new_id();
                let nameexpr = Expr::Name { name: name.clone(), span: span.clone() };
                let expr = match op {
                    BinOp::FloorDiv => UnOp::Floor
                        .to_expr(
                            BinOp::Div.to_expr(nameexpr.into(), index.clone()).into(),
                        )
                        .into(),
                    _ => Expr::BinOp {
                        op: *op,
                        lhs: Expr::BinOp {
                            op: BinOp::Of,
                            lhs: nameexpr.into(),
                            rhs: index.clone(),
                        }
                        .into(),
                        rhs: value.clone(),
                    },
                };
                self.list(s, d, name, span);
                self.input(s, d, "INDEX", &index.borrow(), index_id)?;
                self.input(s, d, "ITEM", &expr, value_id)?;
                self.end_obj()?;
                self.single_field_id("LIST", name)?;
                self.end_obj()?;
                self.expr(s, d, &index.borrow(), index_id, this_id)?;
                self.expr(s, d, &expr, value_id, this_id)?;
            }
            Stmt::Block { block, span, args } => {
                if args.len() != block.args().len() {
                    d.push(
                        DiagnosticKind::BlockArgsCountMismatch {
                            block: *block,
                            given: args.len(),
                        }
                        .to_diagnostic(span.clone()),
                    );
                }
                let arg_ids: Vec<_> = (&mut self.id).take(args.len()).collect();
                let menu_id = block.menu().map(|_| self.id.new_id());
                let mut menu_value = None;
                let mut menu_is_default = menu_id.is_some();
                for ((&name, arg), arg_id) in
                    block.args().iter().zip(args).zip(&arg_ids)
                {
                    if block.menu().is_some_and(|it| it.input == name) {
                        if let Some(arg) = arg.borrow().try_to_string() {
                            menu_value = Some(arg);
                            continue;
                        } else {
                            menu_is_default = false;
                            self.input_with_shadow(
                                s,
                                d,
                                name,
                                &arg.borrow(),
                                *arg_id,
                                menu_id.unwrap(),
                            )?;
                        }
                    } else {
                        self.input(s, d, name, &arg.borrow(), *arg_id)?;
                    }
                }
                if menu_is_default {
                    if self.inputs_comma {
                        self.write_all(b",")?;
                    }
                    self.inputs_comma = true;
                    write!(
                        self,
                        r#""{}":[1,{}]"#,
                        block.menu().unwrap().input,
                        menu_id.unwrap()
                    )?;
                }
                self.end_obj()?;
                if let Some(fields) = block.fields() {
                    write!(self, r#","fields":{fields}"#)?;
                }
                self.end_obj()?;
                for (arg, arg_id) in args.iter().zip(arg_ids) {
                    self.expr(s, d, &arg.borrow(), arg_id, this_id)?;
                }
                if let Some(menu) = block.menu() {
                    self.node(
                        Node::new(menu.opcode, menu_id.unwrap())
                            .parent_id(this_id)
                            .shadow(true),
                    )?;
                    self.single_field(
                        menu.input,
                        menu_value.as_deref().unwrap_or(menu.default),
                    )?;
                    self.end_obj()?;
                }
            }
            Stmt::ProcCall { name, span, args } => {
                let Some(proc) = s.sprite.procs.get(name) else {
                    d.push(
                        DiagnosticKind::UnrecognizedProcedure(name.clone())
                            .to_diagnostic(span.clone()),
                    );
                    return Ok(());
                };
                if args.len() != proc.args.len() {
                    d.push(
                        DiagnosticKind::ProcArgsCountMismatch {
                            proc: name.clone(),
                            given: args.len(),
                        }
                        .to_diagnostic(span.clone()),
                    );
                }
                let arg_ids: Vec<_> = (&mut self.id).take(args.len()).collect();
                for (((name, _), arg), arg_id) in
                    proc.args.iter().zip(args).zip(&arg_ids)
                {
                    self.input(s, d, name, &arg.borrow(), *arg_id)?;
                }
                self.end_obj()?;
                self.mutation()?;
                self.proccode(name, args.len())?;
                self.argument_array("argumentids", &proc.args)?;
                self.warp(proc.warp)?;
                self.end_obj()?;
                self.end_obj()?;
                for (arg, arg_id) in args.iter().zip(arg_ids) {
                    self.expr(s, d, &arg.borrow(), arg_id, this_id)?;
                }
            }
        }
        Ok(())
    }

    fn expr(
        &mut self,
        s: S,
        d: D,
        expr: &Expr,
        this_id: NodeID,
        parent_id: NodeID,
    ) -> Result<()> {
        match expr {
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::Str(_)
            | Expr::Name { .. }
            | Expr::EnumVariant { .. } => {}
            Expr::Arg { name, span } => {
                if !s.is_arg(name) {
                    d.push(
                        DiagnosticKind::UnrecognizedArgument {
                            name: name.clone(),
                            proc: s.proc.map(|proc| proc.name.clone()),
                        }
                        .to_diagnostic(span.clone()),
                    );
                }
                self.node(
                    Node::new("argument_reporter_string_number", this_id)
                        .parent_id(parent_id),
                )?;
                write!(self, r#","fields":{{"VALUE":[{},null]}}}}"#, json!(**name))?;
            }
            Expr::Repr { repr, span, args } => {
                if args.len() != repr.args().len() {
                    d.push(
                        DiagnosticKind::ReprArgsCountMismatch {
                            repr: *repr,
                            given: args.len(),
                        }
                        .to_diagnostic(span.clone()),
                    );
                }
                let arg_ids: Vec<_> = (&mut self.id).take(args.len()).collect();
                self.node(Node::new(repr.opcode(), this_id).parent_id(parent_id))?;
                self.inputs()?;
                let menu_id = repr.menu().map(|_| self.id.new_id());
                let mut menu_value = None;
                let mut menu_is_default = menu_id.is_some();
                for ((&name, arg), arg_id) in repr.args().iter().zip(args).zip(&arg_ids)
                {
                    if repr.menu().is_some_and(|it| it.input == name) {
                        if let Some(arg) = arg.borrow().try_to_string() {
                            menu_value = Some(arg);
                            continue;
                        } else {
                            menu_is_default = false;
                            self.input_with_shadow(
                                s,
                                d,
                                name,
                                &arg.borrow(),
                                *arg_id,
                                menu_id.unwrap(),
                            )?;
                        }
                    } else {
                        self.input(s, d, name, &arg.borrow(), *arg_id)?;
                    }
                }
                if menu_is_default {
                    if self.inputs_comma {
                        self.write_all(b",")?;
                    }
                    self.inputs_comma = true;
                    write!(
                        self,
                        r#""{}":[1,{}]"#,
                        repr.menu().unwrap().input,
                        menu_id.unwrap()
                    )?;
                }
                self.end_obj()?;
                if let Some(fields) = repr.fields() {
                    write!(self, r#","fields":{fields}"#)?;
                }
                self.end_obj()?;
                for (arg, arg_id) in args.iter().zip(arg_ids) {
                    self.expr(s, d, &arg.borrow(), arg_id, this_id)?;
                }
                if let Some(menu) = repr.menu() {
                    self.node(
                        Node::new(menu.opcode, menu_id.unwrap())
                            .parent_id(this_id)
                            .shadow(true),
                    )?;
                    self.single_field(
                        menu.input,
                        menu_value.as_deref().unwrap_or(menu.default),
                    )?;
                    self.end_obj()?;
                }
            }
            Expr::UnOp { op, val } => {
                if matches!(op, UnOp::Length) {
                    if let Expr::Name { name, .. } = &*val.borrow() {
                        if s.sprite.lists.contains_key(name)
                            || s.stage.is_some_and(|it| it.lists.contains_key(name))
                        {
                            self.node(
                                Node::new("data_lengthoflist", this_id)
                                    .parent_id(parent_id),
                            )?;
                            self.single_field_id("LIST", name)?;
                            self.end_obj()?;
                            self.expr(s, d, &val.borrow(), this_id, this_id)?;
                            return Ok(());
                        }
                    }
                }
                let val_id = self.id.new_id();
                self.node(Node::new(op.opcode(), this_id).parent_id(parent_id))?;
                self.inputs()?;
                self.input(s, d, op.input(), &val.borrow(), val_id)?;
                self.end_obj()?;
                if let Some(fields) = op.fields() {
                    write!(self, r#","fields":{fields}"#)?;
                }
                self.end_obj()?;
                self.expr(s, d, &val.borrow(), val_id, this_id)?;
            }
            Expr::BinOp { op, lhs, rhs } => {
                let right_id = self.id.new_id();
                if matches!(op, BinOp::Of) {
                    if let Expr::Name { name, .. } = &*lhs.borrow() {
                        if s.sprite.lists.contains_key(name)
                            || s.stage.is_some_and(|it| it.lists.contains_key(name))
                        {
                            self.node(
                                Node::new("data_itemoflist", this_id)
                                    .parent_id(parent_id),
                            )?;
                            self.inputs()?;
                            self.input(s, d, "INDEX", &rhs.borrow(), right_id)?;
                            self.end_obj()?;
                            self.single_field_id("LIST", name)?;
                            self.end_obj()?;
                            self.expr(s, d, &rhs.borrow(), right_id, this_id)?;
                            return Ok(());
                        }
                    }
                }
                let left_id = self.id.new_id();
                self.node(Node::new(op.opcode(), this_id).parent_id(parent_id))?;
                self.inputs()?;
                self.input(s, d, op.lhs(), &lhs.borrow(), left_id)?;
                self.input(s, d, op.rhs(), &rhs.borrow(), right_id)?;
                self.end_obj()?;
                self.end_obj()?;
                self.expr(s, d, &lhs.borrow(), left_id, this_id)?;
                self.expr(s, d, &rhs.borrow(), right_id, this_id)?;
            }
        }
        Ok(())
    }

    fn list(&mut self, s: S, d: D, name: &SmolStr, span: &Span) {
        if s.sprite.lists.contains_key(name)
            || s.stage.is_some_and(|it| it.lists.contains_key(name))
        {
            return;
        }
        d.push(
            DiagnosticKind::UnrecognizedList(name.clone()).to_diagnostic(span.clone()),
        );
    }

    fn inputs(&mut self) -> io::Result<()> {
        self.inputs_comma = false;
        self.write_all(br#","inputs":{"#)
    }

    fn input(
        &mut self,
        s: S,
        d: D,
        name: &str,
        expr: &Expr,
        this_id: NodeID,
    ) -> io::Result<()> {
        self._input(s, d, name, expr, this_id, None)
    }

    fn input_with_shadow(
        &mut self,
        s: S,
        d: D,
        name: &'static str,
        expr: &Expr,
        this_id: NodeID,
        shadow_id: NodeID,
    ) -> io::Result<()> {
        self._input(s, d, name, expr, this_id, Some(shadow_id))
    }

    fn substack(
        &mut self,
        name: &'static str,
        this_id: Option<NodeID>,
    ) -> io::Result<()> {
        if let Some(this_id) = this_id {
            if self.inputs_comma {
                self.write_all(b",")?;
            }
            self.inputs_comma = true;
            write!(self, r#""{name}":[2,{this_id}]"#)?;
        }
        Ok(())
    }

    fn _input(
        &mut self,
        s: S,
        d: D,
        name: &str,
        expr: &Expr,
        this_id: NodeID,
        shadow_id: Option<NodeID>,
    ) -> io::Result<()> {
        if self.inputs_comma {
            self.write_all(b",")?;
        }
        self.inputs_comma = true;
        write!(self, r#""{name}":"#)?;
        match expr {
            Expr::Int(value) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))
            }
            Expr::Float(value) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))
            }
            Expr::Str(value) => {
                let color = if name == "COLOR" || name == "COLOR2" {
                    csscolorparser::parse(value).ok().filter(|color| color.a == 1.0)
                } else {
                    None
                };
                if name == "BROADCAST_INPUT" {
                    write!(self, r#"[1,[11,{},{}]]"#, json!(**value), json!(**value))
                } else if let Some(color) = color {
                    write!(self, r#"[1,[9,{}]]"#, json!(color.to_hex_string()))
                } else {
                    write!(self, r#"[1,[10,{}]]"#, json!(**value))
                }
            }
            Expr::EnumVariant { enum_name, enum_span, variant_name, variant_span } => {
                if let Some(enum_) = s.sprite.enums.get(enum_name) {
                    let index = enum_
                        .variants
                        .iter()
                        .position(|(variant, _)| variant == variant_name);
                    if let Some(index) = index {
                        write!(self, r#"[1,[10,{index}]]"#)
                    } else {
                        d.push(
                            DiagnosticKind::UnrecognizedEnumVariant {
                                enum_name: enum_name.clone(),
                                variant_name: variant_name.clone(),
                            }
                            .to_diagnostic(variant_span.clone()),
                        );
                        Ok(())
                    }
                } else {
                    d.push(
                        DiagnosticKind::UnrecognizedEnum {
                            enum_name: enum_name.clone(),
                            variant_name: variant_name.clone(),
                        }
                        .to_diagnostic(enum_span.clone()),
                    );
                    Ok(())
                }
            }
            Expr::Name { name: var, span } => {
                if let Some(resolved) =
                    self.resolve_local_variable(s, var).map(|it| json!(it))
                {
                    write!(self, "[3,[12,{},{}],", resolved, resolved)?;
                } else if s.is_var(var) {
                    write!(self, "[3,[12,{},{}],", json!(**var), json!(**var))?;
                } else if s.is_list(var) {
                    write!(self, "[3,[13,{},{}],", json!(**var), json!(**var))?;
                } else {
                    d.push(
                        DiagnosticKind::UnrecognizedVariable(var.clone())
                            .to_diagnostic(span.clone()),
                    );
                }
                self.input_shadow(s, shadow_id, name)
            }
            _ => {
                if name == "CONDITION" || name == "CONDITION2" {
                    return write!(self, r#"[2,{this_id}]"#);
                }
                write!(self, r#"[3,{this_id},"#)?;
                self.input_shadow(s, shadow_id, name)
            }
        }
    }

    fn input_shadow(
        &mut self,
        s: S,
        shadow_id: Option<NodeID>,
        name: &str,
    ) -> io::Result<()> {
        if let Some(shadow_id) = shadow_id {
            write!(self, "{shadow_id}]")
        } else if name == "BROADCAST_INPUT" {
            let broadcast_name = match s.stage {
                Some(stage) => {
                    stage.broadcasts.iter().min().expect("no broadcasts?").clone()
                }
                None => "message1".into(),
            };
            write!(
                self,
                r#"[11,{},{}]]"#,
                json!(*broadcast_name),
                json!(*broadcast_name)
            )
        } else {
            self.write_all(br#"[10,""]]"#)
        }
    }

    fn single_field(&mut self, name: &'static str, value: &str) -> io::Result<()> {
        write!(self, r#","fields":{{"{name}":[{},null]}}"#, json!(value))
    }

    fn resolve_local_variable(&mut self, s: S, name: &SmolStr) -> Option<String> {
        s.proc.and_then(|proc| {
            proc.locals
                .contains_key(name)
                .then(|| local_variable_resolved_name(proc, name))
        })
    }

    fn resolve_variable(
        &mut self,
        s: S,
        d: D,
        name: &SmolStr,
        span: &Span,
    ) -> io::Result<()> {
        if s.is_local_var(name) {
            return self.single_field(
                "VARIABLE",
                &local_variable_resolved_name(s.proc.unwrap(), name),
            );
        }
        if s.is_var(name) {
            return self.single_field_id("VARIABLE", name);
        }
        d.push(
            DiagnosticKind::UnrecognizedVariable(name.clone())
                .to_diagnostic(span.clone()),
        );
        Ok(())
    }

    fn resolve_variable_or_list(
        &mut self,
        s: S,
        d: D,
        name: &SmolStr,
        span: &Span,
    ) -> io::Result<()> {
        if s.is_local_var(name) {
            return self.single_field(
                "VARIABLE",
                &local_variable_resolved_name(s.proc.unwrap(), name),
            );
        }
        if s.is_var(name) {
            return self.single_field_id("VARIABLE", name);
        }
        if s.is_list(name) {
            return self.single_field_id("LIST", name);
        }
        d.push(
            DiagnosticKind::UnrecognizedVariable(name.clone())
                .to_diagnostic(span.clone()),
        );
        Ok(())
    }

    fn single_field_id(&mut self, name: &'static str, value: &str) -> io::Result<()> {
        write!(self, r#","fields":{{"{name}":[{},{}]}}"#, json!(value), json!(value))
    }

    fn mutation(&mut self) -> io::Result<()> {
        self.write_all(br#","mutation":{"tagName":"mutation","children":[]"#)
    }

    fn warp(&mut self, warp: bool) -> io::Result<()> {
        if warp {
            self.write_all(br#","warp":"true""#)
        } else {
            self.write_all(br#","warp":"false""#)
        }
    }

    fn argument_array(
        &mut self,
        key: &'static str,
        args: &Vec<(SmolStr, Span)>,
    ) -> io::Result<()> {
        write!(self, r#","{key}":"["#)?;
        let mut comma = false;
        for (arg, _) in args {
            self.comma(&mut comma)?;
            write!(self, r#"\"{arg}\""#)?;
        }
        self.write_all(br#"]""#)
    }

    fn proccode(&mut self, name: &str, args: usize) -> io::Result<()> {
        write!(self, r#","proccode":"{name}"#)?;
        for _ in 0..args {
            self.write_all(b" %s")?;
        }
        self.write_all(br#"""#)
    }

    fn comma(&mut self, comma: &mut bool) -> io::Result<()> {
        if *comma {
            self.write_all(b",")?;
        }
        *comma = true;
        Ok(())
    }

    fn end_obj(&mut self) -> io::Result<()> {
        self.write_all(b"}")
    }
}

impl<T> Write for Sb3<T>
where T: Write + Seek
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.zip.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.zip.flush()
    }
}

fn local_variable_resolved_name(proc: &Proc, name: &SmolStr) -> String {
    format!("{}.{}", proc.name, name)
}
