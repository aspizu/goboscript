use core::str;
use std::{
    fs::File,
    io::{self, Seek, Write},
    path::Path,
    process::Command,
};

use fxhash::FxHashMap;
use logos::Span;
use md5::{Digest, Md5};
use serde_json::json;
use smol_str::SmolStr;
use zip::{write::SimpleFileOptions, ZipWriter};

use super::{
    cmd::cmd_to_list, node::Node, node_id::NodeID, node_id_factory::NodeIDFactory,
    turbowarp_config::TurbowarpConfig,
};
use crate::{
    ast::*,
    blocks::Block,
    codegen::mutation::Mutation,
    config::Config,
    diagnostic::{DiagnosticKind, SpriteDiagnostics},
    misc::write_comma_io,
};

const STAGE_NAME: &str = "Stage";

#[derive(Debug, Copy, Clone)]
pub struct S<'a> {
    pub stage: Option<&'a Sprite>,
    pub sprite: &'a Sprite,
    pub proc: Option<&'a Proc>,
}

pub type D<'a> = &'a mut SpriteDiagnostics;

pub enum QualifiedName {
    Var(SmolStr, Type),
    List(SmolStr, Type),
}

pub fn qualify_local_var_name(proc_name: &str, var_name: &str) -> SmolStr {
    format!("{}:{}", proc_name, var_name).into()
}

pub fn qualify_struct_var_name(field_name: &str, var_name: &str) -> SmolStr {
    format!("{}.{}", var_name, field_name).into()
}

impl<'a> S<'a> {
    pub fn is_name_list(&self, name: &Name) -> bool {
        self.sprite.lists.contains_key(name.basename())
            || self
                .stage
                .is_some_and(|stage| stage.lists.contains_key(name.basename()))
    }

    fn get_local_var(&self, name: &str) -> Option<&Var> {
        self.proc.and_then(|proc| proc.locals.get(name))
    }

    fn get_var(&self, name: &str) -> Option<&Var> {
        self.sprite
            .vars
            .get(name)
            .or_else(|| self.stage.and_then(|stage| stage.vars.get(name)))
    }

    fn get_list(&self, name: &str) -> Option<&List> {
        self.sprite
            .lists
            .get(name)
            .or_else(|| self.stage.and_then(|stage| stage.lists.get(name)))
    }

    fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.sprite
            .structs
            .get(name)
            .or_else(|| self.stage.and_then(|stage| stage.structs.get(name)))
    }

    fn qualify_field<T>(
        &self,
        d: D,
        span: &Span,
        qualified_var_name: SmolStr,
        field_name: Option<SmolStr>,
        type_: &Type,
        variant: T,
    ) -> Option<QualifiedName>
    where
        T: FnOnce(SmolStr, Type) -> QualifiedName,
    {
        match type_ {
            Type::Value => match field_name {
                None => Some(variant(qualified_var_name, type_.clone())),
                Some(_) => {
                    d.report(DiagnosticKind::NotStruct, span);
                    None
                }
            },
            Type::Struct {
                name: type_name,
                span: type_span,
            } => match field_name {
                None => panic!("attempted to qualify struct var without field name, type error?"),
                Some(field_name) => {
                    let struct_ = self.get_struct(type_name)?;
                    if !struct_.fields.iter().any(|field| field.name == field_name) {
                        d.report(
                            DiagnosticKind::StructDoesNotHaveField {
                                type_name: type_name.clone(),
                                field_name: field_name.clone(),
                            },
                            type_span,
                        );
                        None
                    } else {
                        Some(variant(
                            qualify_struct_var_name(&field_name, &qualified_var_name),
                            type_.clone(),
                        ))
                    }
                }
            },
        }
    }

    pub fn qualify_name(&self, d: D, name: &Name) -> Option<QualifiedName> {
        let basename = name.basename();
        let fieldname = name.fieldname().cloned();
        if let Some(list) = self.get_list(basename) {
            return self.qualify_field(
                d,
                &name.span(),
                list.name.clone(),
                fieldname,
                &list.type_,
                QualifiedName::List,
            );
        }
        if let Some(var) = self.get_local_var(basename) {
            let qualified_var_name = qualify_local_var_name(&self.proc.unwrap().name, &var.name);
            return self.qualify_field(
                d,
                &name.span(),
                qualified_var_name,
                fieldname,
                &var.type_,
                QualifiedName::Var,
            );
        }
        if let Some(var) = self.get_var(basename) {
            return self.qualify_field(
                d,
                &name.span(),
                var.name.clone(),
                fieldname,
                &var.type_,
                QualifiedName::Var,
            );
        }
        d.report(
            DiagnosticKind::UnrecognizedVariable(basename.clone()),
            &name.span(),
        );
        None
    }
}

impl Stmt {
    fn is_terminator(&self) -> bool {
        matches!(
            self,
            Stmt::Forever { .. }
                | Stmt::Block {
                    block: Block::DeleteThisClone | Block::StopAll | Block::StopThisScript,
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
            Stmt::Show(name) => {
                if s.is_name_list(name) {
                    "data_showlist"
                } else {
                    "data_showvariable"
                }
            }
            Stmt::Hide(name) => {
                if s.is_name_list(name) {
                    "data_hidelist"
                } else {
                    "data_hidevariable"
                }
            }
            Stmt::AddToList { .. } => "data_addtolist",
            Stmt::DeleteListIndex { .. } => "data_deleteoflist",
            Stmt::DeleteList { .. } => "data_deletealloflist",
            Stmt::InsertAtList { .. } => "data_insertatlist",
            Stmt::SetListIndex { .. } => "data_replaceitemoflist",
            Stmt::Block { block, .. } => block.opcode(),
            Stmt::ProcCall { .. } => "procedures_call",
        }
    }
}

#[derive(Debug)]
pub struct Sb3<T>
where T: Write + Seek
{
    pub zip: ZipWriter<T>,
    pub id: NodeIDFactory,
    pub node_comma: bool,
    pub inputs_comma: bool,
    pub costumes: FxHashMap<SmolStr, SmolStr>,
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

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn new(file: T) -> Self {
        Self {
            zip: ZipWriter::new(file),
            id: NodeIDFactory::new(),
            node_comma: false,
            inputs_comma: false,
            costumes: FxHashMap::default(),
        }
    }

    pub fn begin_node(&mut self, node: Node) -> io::Result<()> {
        write_comma_io(&mut self.zip, &mut self.node_comma)?;
        write!(self, "{node}")
    }

    pub fn end_obj(&mut self) -> io::Result<()> {
        self.write_all(b"}")
    }

    pub fn begin_inputs(&mut self) -> io::Result<()> {
        self.inputs_comma = false;
        self.write_all(br#","inputs":{"#)
    }

    pub fn single_field(&mut self, name: &'static str, value: &str) -> io::Result<()> {
        write!(self, r#","fields":{{"{name}":[{},null]}}"#, json!(value))
    }

    pub fn single_field_id(&mut self, name: &'static str, value: &str) -> io::Result<()> {
        write!(
            self,
            r#","fields":{{"{name}":[{},{}]}}"#,
            json!(value),
            json!(value)
        )
    }

    pub fn substack(&mut self, name: &str, this_id: Option<NodeID>) -> io::Result<()> {
        let Some(this_id) = this_id else {
            return Ok(());
        };
        write_comma_io(&mut self.zip, &mut self.inputs_comma)?;
        write!(self, r#""{name}":[2,{this_id}]"#)
    }

    pub fn project(
        &mut self,
        input: &Path,
        project: &Project,
        config: &Config,
        stage_diagnostics: D,
        sprites_diagnostics: &mut FxHashMap<SmolStr, SpriteDiagnostics>,
    ) -> io::Result<()> {
        // TODO: switch to deflate compression
        // this should be configurable, use store in debug (because it would be
        // faster?), use deflate in release (because it would be smaller?)
        self.zip
            .start_file("project.json", SimpleFileOptions::default())?;
        write!(self, "{{")?;
        write!(self, r#""targets":["#)?;
        self.sprite(
            input,
            STAGE_NAME,
            &project.stage,
            None,
            config,
            stage_diagnostics,
        )?;
        for (sprite_name, sprite) in &project.sprites {
            write!(self, r#","#)?;
            self.sprite(
                input,
                sprite_name,
                sprite,
                Some(&project.stage),
                config,
                sprites_diagnostics.get_mut(sprite_name).unwrap(),
            )?;
        }
        write!(self, "]")?; // targets
        write!(self, r#","monitors":[]"#)?;
        write!(self, r#","extensions":[]"#)?;
        write!(self, r#","meta":{{"#)?;
        write!(self, r#""semver":"3.0.0""#)?;
        write!(self, r#","vm":"0.2.0""#)?;
        write!(
            self,
            r#","agent":"goboscript v{}""#,
            env!("CARGO_PKG_VERSION")
        )?;
        write!(self, "}}")?; // meta
        write!(self, "}}")?; // project
        Ok(())
    }

    pub fn sprite(
        &mut self,
        input: &Path,
        name: &str,
        sprite: &Sprite,
        stage: Option<&Sprite>,
        config: &Config,
        d: D,
    ) -> io::Result<()> {
        self.id.reset();
        write!(self, "{{")?;
        write!(self, r#""isStage":{}"#, name == STAGE_NAME)?;
        write!(self, r#","name":{}"#, json!(name))?;
        if name == STAGE_NAME {
            write!(self, r#","comments":{{"#)?;
            write!(self, r#""twconfig":{{"#)?;
            write!(self, r#""blockId":null"#)?;
            write!(self, r#","x":0"#)?;
            write!(self, r#","y":0"#)?;
            write!(self, r#","width":350"#)?;
            write!(self, r#","height":170"#)?;
            write!(self, r#","minimized":false"#)?;
            write!(
                self,
                r#","text":{}"#,
                json!(TurbowarpConfig::from(config).to_string())
            )?;
            write!(self, "}}")?; // twconfig
            write!(self, "}}")?; // comments
        }
        write!(self, r#","variables":{{"#)?;
        let mut comma = false;
        for proc in sprite.procs.values() {
            for var in proc.locals.values() {
                self.local_var_declaration(sprite, proc, var, &mut comma, d)?;
            }
        }
        for var in sprite.vars.values() {
            self.var_declaration(sprite, var, &mut comma, d)?;
        }
        write!(self, "}}")?; // variables
        write!(self, r#","lists":{{"#)?;
        let mut comma = false;
        for list in sprite.lists.values() {
            self.list_declaration(input, sprite, list, &mut comma, d)?;
        }
        write!(self, "}}")?; // lists
        write!(self, r#","blocks":{{"#)?;
        self.node_comma = false;
        for proc in sprite.procs.values() {
            self.proc(
                S {
                    stage,
                    sprite,
                    proc: Some(proc),
                },
                d,
                proc,
            )?;
        }
        for event in &sprite.events {
            self.event(
                S {
                    stage,
                    sprite,
                    proc: None,
                },
                d,
                event,
            )?;
        }
        write!(self, "}}")?; // blocks
        write!(self, r#","costumes":["#)?;
        let mut comma = false;
        for costume in &sprite.costumes {
            write_comma_io(&mut self.zip, &mut comma)?;
            self.costume(input, costume, d)?;
        }
        write!(self, "]")?; // costumes
        write!(self, r#","sounds":["#)?;
        write!(self, "]")?; // sounds
        write!(self, "}}")?; // sprite
        Ok(())
    }

    pub fn json_var_declaration(&mut self, var_name: &str, comma: &mut bool) -> io::Result<()> {
        write_comma_io(&mut self.zip, comma)?;
        write!(self, r#""{}":["{}",0]"#, var_name, var_name)
    }

    pub fn var_declaration(
        &mut self,
        sprite: &Sprite,
        var: &Var,
        comma: &mut bool,
        d: D,
    ) -> io::Result<()> {
        match &var.type_ {
            Type::Value => {
                self.json_var_declaration(&var.name, comma)?;
            }
            Type::Struct {
                name: type_name,
                span: type_span,
            } => {
                let Some(struct_) = sprite.structs.get(type_name) else {
                    d.report(
                        DiagnosticKind::UnrecognizedStruct(type_name.clone()),
                        type_span,
                    );
                    return Ok(());
                };
                for field in &struct_.fields {
                    let qualified_var_name = qualify_struct_var_name(&field.name, &var.name);
                    self.json_var_declaration(&qualified_var_name, comma)?;
                }
            }
        }
        Ok(())
    }

    pub fn local_var_declaration(
        &mut self,
        sprite: &Sprite,
        proc: &Proc,
        var: &Var,
        comma: &mut bool,
        d: D,
    ) -> io::Result<()> {
        match &var.type_ {
            Type::Value => {
                let qualified_var_name = qualify_local_var_name(&proc.name, &var.name);
                self.json_var_declaration(&qualified_var_name, comma)?;
            }
            Type::Struct {
                name: type_name,
                span: type_span,
            } => {
                let Some(struct_) = sprite.structs.get(type_name) else {
                    d.report(
                        DiagnosticKind::UnrecognizedStruct(type_name.clone()),
                        type_span,
                    );
                    return Ok(());
                };
                for field in &struct_.fields {
                    let qualified_var_name = qualify_local_var_name(
                        &proc.name,
                        &qualify_struct_var_name(&field.name, &var.name),
                    );
                    self.json_var_declaration(&qualified_var_name, comma)?;
                }
            }
        }
        Ok(())
    }

    pub fn list_declaration(
        &mut self,
        input: &Path,
        sprite: &Sprite,
        list: &List,
        comma: &mut bool,
        d: D,
    ) -> io::Result<()> {
        let data = list.cmd.as_ref().and_then(|cmd| {
            cmd_to_list(cmd, input)
                .map_err(|err| d.diagnostics.push(err))
                .ok()
        });
        match &list.type_ {
            Type::Value => {
                write_comma_io(&mut self.zip, comma)?;
                if let Some(cmd) = data {
                    write!(self, r#""{}":["{}",{}]"#, list.name, list.name, json!(cmd))?;
                } else {
                    write!(self, r#""{}":["{}",[]]"#, list.name, list.name)?;
                }
            }
            Type::Struct {
                name: type_name,
                span: type_span,
            } => {
                let Some(struct_) = sprite.structs.get(type_name) else {
                    d.report(
                        DiagnosticKind::UnrecognizedStruct(type_name.clone()),
                        type_span,
                    );
                    return Ok(());
                };
                for (i, field) in struct_.fields.iter().enumerate() {
                    let qualified_list_name = qualify_struct_var_name(&field.name, &list.name);
                    write_comma_io(&mut self.zip, comma)?;
                    if let Some(cmd) = &data {
                        let column = (0..(cmd.len() / struct_.fields.len()))
                            .map(|j| &cmd[j * struct_.fields.len() + i])
                            .collect::<Vec<_>>();
                        write!(
                            self,
                            r#""{}":["{}",{}]"#,
                            qualified_list_name,
                            qualified_list_name,
                            json!(column)
                        )?;
                    } else {
                        write!(
                            self,
                            r#""{}":["{}",[]]"#,
                            qualified_list_name, qualified_list_name
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn costume(&mut self, input: &Path, costume: &Costume, d: D) -> io::Result<()> {
        let path = input.join(&costume.path);
        let hash = self
            .costumes
            .get(&costume.path)
            .cloned()
            .map(Ok::<_, io::Error>)
            .unwrap_or_else(|| {
                if !path.is_file() {
                    d.report(
                        DiagnosticKind::FileNotFound(costume.path.clone()),
                        &costume.span,
                    );
                    return Ok(Default::default());
                }
                let mut file = File::open(&path)?;
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher)?;
                let hash: SmolStr = format!("{:x}", hasher.finalize()).into();
                self.costumes.insert(costume.path.clone(), hash.clone());
                Ok(hash)
            })?;
        let (_, extension) = costume.path.rsplit_once('.').unwrap();
        write!(self, "{{")?;
        write!(self, r#""name":{}"#, json!(*costume.name))?;
        write!(self, r#","assetId":"{hash}""#)?;
        write!(self, r#","dataFormat":"{extension}""#)?;
        write!(self, r#","md5ext":"{hash}.{extension}""#)?;
        write!(self, "}}") // costume
    }

    pub fn proc(&mut self, s: S, d: D, proc: &Proc) -> io::Result<()> {
        let this_id = self.id.new_id();
        let prototype_id = self.id.new_id();
        let next_id = self.id.new_id();
        self.begin_node(
            Node::new("procedures_definition", this_id)
                .some_next_id((!proc.body.is_empty()).then_some(next_id))
                .top_level(true),
        )?;
        self.begin_inputs()?;
        write!(self, r#""custom_block":[1,{prototype_id}]"#)?;
        self.end_obj()?; // inputs
        self.end_obj()?; // node
        let mut qualified_args: Vec<(SmolStr, NodeID)> = Vec::new();
        for arg in &proc.args {
            match &arg.type_ {
                Type::Value => {
                    let arg_id = self.id.new_id();
                    self.begin_node(
                        Node::new("argument_reporter_string_number", arg_id)
                            .parent_id(prototype_id)
                            .shadow(true),
                    )?;
                    self.single_field("VALUE", &arg.name)?;
                    self.end_obj()?; // node
                    qualified_args.push((arg.name.clone(), arg_id));
                }
                Type::Struct {
                    name: type_name,
                    span: type_span,
                } => {
                    let Some(struct_) = s.sprite.structs.get(type_name) else {
                        d.report(
                            DiagnosticKind::UnrecognizedStruct(type_name.clone()),
                            type_span,
                        );
                        continue;
                    };
                    for field in &struct_.fields {
                        let qualified_arg_name = qualify_struct_var_name(&field.name, &arg.name);
                        let arg_id = self.id.new_id();
                        self.begin_node(
                            Node::new("argument_reporter_string_number", arg_id)
                                .parent_id(prototype_id)
                                .shadow(true),
                        )?;
                        self.single_field("VALUE", &qualified_arg_name)?;
                        self.end_obj()?; // node
                        qualified_args.push((qualified_arg_name, arg_id));
                    }
                }
            }
        }
        self.begin_node(
            Node::new("procedures_prototype", prototype_id)
                .parent_id(this_id)
                .shadow(true),
        )?;
        self.begin_inputs()?;
        let mut comma = false;
        for (qualified_arg_name, arg_id) in &qualified_args {
            write_comma_io(&mut self.zip, &mut comma)?;
            write!(self, r#"{}:[2,{arg_id}]"#, json!(**qualified_arg_name))?;
        }
        self.end_obj()?; // inputs
        write!(
            self,
            "{}",
            Mutation::prototype(proc.name.clone(), &qualified_args, proc.warp,)
        )?;
        self.end_obj()?; // node
        self.stmts(s, d, &proc.body, next_id, Some(this_id))
    }

    pub fn event(&mut self, s: S, d: D, event: &Event) -> io::Result<()> {
        let this_id = self.id.new_id();
        let next_id = self.id.new_id();
        self.begin_node(
            Node::new(event.kind.opcode(), this_id)
                .some_next_id((!event.body.is_empty()).then_some(next_id))
                .top_level(true),
        )?;
        match &event.kind {
            EventKind::OnFlag => self.on_flag(s, d, this_id),
            EventKind::OnKey { key, span } => self.on_key(s, d, this_id, key, span),
            EventKind::OnClick => self.on_click(s, d, this_id),
            EventKind::OnBackdrop { backdrop, span } => {
                self.on_backdrop(s, d, this_id, backdrop, span)
            }
            EventKind::OnLoudnessGt { value } => self.on_loudness_gt(s, d, this_id, value),
            EventKind::OnTimerGt { value } => self.on_timer_gt(s, d, this_id, value),
            EventKind::OnClone => self.on_clone(s, d, this_id),
        }?;
        self.stmts(s, d, &event.body, next_id, Some(this_id))
    }

    pub fn stmts(
        &mut self,
        s: S,
        d: D,
        stmts: &[Stmt],
        mut this_id: NodeID,
        mut parent_id: Option<NodeID>,
    ) -> io::Result<()> {
        for (i, stmt) in stmts.iter().enumerate() {
            let is_last = i == stmts.len() - 1;
            if is_last || stmt.is_terminator() {
                self.stmt(s, d, stmt, this_id, None, parent_id)?;
                if !is_last {
                    d.report(DiagnosticKind::FollowedByUnreachableCode, stmt.span());
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

    pub fn stmt(
        &mut self,
        s: S,
        d: D,
        stmt: &Stmt,
        this_id: NodeID,
        next_id: Option<NodeID>,
        parent_id: Option<NodeID>,
    ) -> io::Result<()> {
        self.begin_node(
            Node::new(stmt.opcode(s), this_id)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        match stmt {
            Stmt::Repeat { times, body } => self.repeat(s, d, this_id, times, body),
            Stmt::Forever { body, span } => self.forever(s, d, this_id, body, span),
            Stmt::Branch {
                cond,
                if_body,
                else_body,
            } => self.branch(s, d, this_id, cond, if_body, else_body),
            Stmt::Until { cond, body } => self.until(s, d, this_id, cond, body),
            Stmt::SetVar {
                name,
                value,
                type_,
                is_local,
            } => self.set_var(s, d, this_id, name, value, type_, is_local),
            Stmt::ChangeVar { name, value } => self.change_var(s, d, this_id, name, value),
            Stmt::Show(name) => self.show(s, d, name),
            Stmt::Hide(name) => self.hide(s, d, name),
            Stmt::AddToList { name, value } => self.add_to_list(s, d, this_id, name, value),
            Stmt::DeleteListIndex { name, index } => {
                self.delete_list_index(s, d, this_id, name, index)
            }
            Stmt::DeleteList(name) => self.delete_list(s, d, name),
            Stmt::InsertAtList { name, index, value } => {
                self.list_insert(s, d, this_id, name, index, value)
            }
            Stmt::SetListIndex { name, index, value } => {
                self.set_list_index(s, d, this_id, name, index, value)
            }
            Stmt::Block { block, span, args } => self.block(s, d, this_id, block, span, args),
            Stmt::ProcCall { name, span, args } => self.proc_call(s, d, this_id, name, span, args),
        }
    }

    pub fn expr(
        &mut self,
        s: S,
        d: D,
        expr: &Expr,
        this_id: NodeID,
        parent_id: NodeID,
    ) -> io::Result<()> {
        match expr {
            Expr::Value { .. } => Ok(()),
            Expr::Name { .. } => Ok(()),
            Expr::Arg(name) => self.arg(s, d, this_id, parent_id, name),
            Expr::Repr { repr, span, args } => {
                self.repr(s, d, this_id, parent_id, repr, span, args)
            }
            Expr::UnOp { op, span, opr } => self.un_op(s, d, this_id, parent_id, op, span, opr),
            Expr::BinOp { op, span, lhs, rhs } => {
                self.bin_op(s, d, this_id, parent_id, op, span, lhs, rhs)
            }
            Expr::StructLiteral { name, span, .. } => {
                d.report(
                    DiagnosticKind::TypeMismatch {
                        expected: Type::Value,
                        given: Type::Struct {
                            name: name.clone(),
                            span: span.clone(),
                        },
                    },
                    &expr.span(),
                );
                Ok(())
            }
            Expr::Dot { .. } => panic!("Attempted to codegen {expr:#?}"),
        }
    }
}
