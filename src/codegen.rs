use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, Seek, Write},
    path::PathBuf,
    rc::Rc,
};

use logos::Span;
use md5::{Digest, Md5};
use serde_json::json;

use crate::{
    ast::{
        BinaryOp, Block, Declr, Declrs, Expr, Exprs, Names, Procedure, Reporter, Rrc,
        Stmt, Stmts, UnaryOp,
    },
    blockid::{BlockID, BlockIDFactory},
    build::Program,
    config::Config,
    details::{block_details, reporter_details},
    reporting::Report,
    visitors::ProcedurePrototype,
    zipfile::ZipFile,
};

pub const KEYS: &[&str] =
    &["space", "up arrow", "down arrow", "left arrow", "right arrow", "any"];

struct Node {
    opcode: &'static str,
    this_id: BlockID,
    comma: bool,
    next_id: Option<BlockID>,
    parent_id: Option<BlockID>,
    top_level: bool,
    shadow: bool,
}

impl Node {
    fn new(opcode: &'static str, this_id: BlockID, comma: bool) -> Self {
        Self {
            opcode,
            this_id,
            comma,
            next_id: None,
            parent_id: None,
            top_level: false,
            shadow: false,
        }
    }

    fn next_id(mut self, next_id: BlockID) -> Self {
        self.next_id = Some(next_id);
        self
    }

    fn some_next_id(mut self, next_id: Option<BlockID>) -> Self {
        self.next_id = next_id;
        self
    }

    fn parent_id(mut self, parent_id: BlockID) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    fn some_parent_id(mut self, parent_id: Option<BlockID>) -> Self {
        self.parent_id = parent_id;
        self
    }

    fn top_level(mut self) -> Self {
        self.top_level = true;
        self
    }

    fn shadow(mut self) -> Self {
        self.shadow = true;
        self
    }
}

pub struct CodeGen<T>
where
    T: Write + Seek,
{
    zip: ZipFile<T>,
    id: BlockIDFactory,
    costumes: HashMap<Rc<str>, String>,
    input: PathBuf,
    pub config: Config,
}

type R<'src, 'b> = &'b mut Vec<Report<'src>>;

#[derive(Clone, Copy)]
pub struct Sc<'src, 'b> {
    variables: &'b HashSet<&'src str>,
    global_variables: Option<&'b HashSet<&'src str>>,
    lists: &'b HashSet<&'src str>,
    global_lists: Option<&'b HashSet<&'src str>>,
    procedures: &'b HashMap<&'src str, ProcedurePrototype<'src>>,
    procedure: Option<&'b ProcedurePrototype<'src>>,
}

impl<'src, T> CodeGen<T>
where
    T: Write + Seek,
{
    pub fn new(zip: ZipFile<T>, input: PathBuf, config: Config) -> CodeGen<T> {
        CodeGen {
            zip,
            id: Default::default(),
            costumes: Default::default(),
            input,
            config,
        }
    }

    fn comma(&mut self, comma: bool) -> io::Result<()> {
        if comma {
            self.write_all(",".as_bytes())?;
        }
        Ok(())
    }

    fn begin_object(&mut self) -> io::Result<()> {
        self.write_all("{".as_bytes())
    }

    fn end_object(&mut self) -> io::Result<()> {
        self.write_all("}".as_bytes())
    }

    fn begin_array(&mut self) -> io::Result<()> {
        self.write_all("[".as_bytes())
    }

    fn end_array(&mut self) -> io::Result<()> {
        self.write_all("]".as_bytes())
    }

    fn key(&mut self, key: &str) -> io::Result<()> {
        write!(self, r#""{key}":"#)
    }

    fn string(&mut self, s: &'static str) -> io::Result<()> {
        write!(self, r#""{s}""#)
    }

    fn begin_node(&mut self, node: Node) -> io::Result<()> {
        self.comma(node.comma)?;
        write!(self, r#"{}:{{"#, node.this_id)?;
        self.key("opcode")?;
        self.string(node.opcode)?;
        if let Some(next_id) = node.next_id {
            self.comma(true)?;
            self.key("next")?;
            write!(self, "{next_id}")?;
        }
        if let Some(parent_id) = node.parent_id {
            self.comma(true)?;
            self.key("parent")?;
            write!(self, "{parent_id}")?;
        }
        if node.top_level {
            self.comma(true)?;
            self.key("topLevel")?;
            self.write_all("true".as_bytes())?;
        }
        if node.shadow {
            self.comma(true)?;
            self.key("shadow")?;
            self.write_all("true".as_bytes())?;
        }
        Ok(())
    }

    pub fn begin_project(&mut self) -> io::Result<()> {
        self.zip.begin_file("project.json")?;
        self.write_all(r#"{"targets":["#.as_bytes())
    }

    pub fn end_project(&mut self) -> io::Result<()> {
        self.write_all(r#"],"monitors":[],"extensions":[],"meta":{"semver":"3.0.0","vm":"0.2.0","agent":"goboscript"}}"#.as_bytes())?;
        self.write_assets()?;
        self.zip.end_zip()
    }

    fn write_assets(&mut self) -> io::Result<()> {
        for (costume, hash) in &self.costumes {
            let (_, extension) = costume.rsplit_once('.').unwrap();
            self.zip.begin_file(&format!("{hash}.{extension}"))?;
            let mut file = File::open(self.input.join(costume.as_ref()))?;
            io::copy(&mut file, &mut self.zip)?;
        }
        Ok(())
    }

    pub fn sprite(
        &mut self,
        name: &str,
        program: &Program<'src>,
        global_variables: Option<&'src HashSet<&'src str>>,
        global_lists: Option<&'src HashSet<&'src str>>,
        reports: &mut Vec<Report<'src>>,
        comma: bool,
    ) -> io::Result<()> {
        self.id.reset();
        self.comma(comma)?;
        self.begin_object()?;
        self.key("name")?;
        write!(self, "{}", json!(name))?;
        self.comma(true)?;
        self.key("isStage")?;
        if name == "Stage" {
            self.write_all("true".as_bytes())?;
            if !self.config.is_default() {
                self.comma(true)?;
                self.key("comments")?;
                self.begin_object()?;
                self.key("a")?;
                self.begin_object()?;
                self.key("blockId")?;
                self.write_all(b"null")?;
                self.comma(true)?;
                self.key("x")?;
                self.write_all(b"0")?;
                self.comma(true)?;
                self.key("y")?;
                self.write_all(b"0")?;
                self.comma(true)?;
                self.key("width")?;
                self.write_all(b"350")?;
                self.comma(true)?;
                self.key("height")?;
                self.write_all(b"170")?;
                self.comma(true)?;
                self.key("minimized")?;
                self.write_all(b"true")?;
                self.comma(true)?;
                self.key("text")?;
                write!(
                    self, // FIXME
                    "{}",
                    json!(format!(
                        r#"{} // _twconfig_"#,
                        json!({
                            "framerate": self.config.frame_rate,
                            "runtimeOptions": {
                                "maxClones": self.config.max_clones,
                                "miscLimits": self.config.miscellaneous_limits,
                                "fencing": self.config.sprite_fencing,
                            },
                            "interpolation": self.config.frame_interpolation,
                            "hq": self.config.high_quality_pen,
                            "width": self.config.stage_width,
                            "height": self.config.stage_height,
                        })
                    ))
                )?;
                self.end_object()?;
                self.end_object()?;
            }
        } else {
            self.write_all("false".as_bytes())?;
        }
        self.comma(true)?;
        self.key("blocks")?;
        self.begin_object()?;
        self.declrs(
            reports,
            Sc {
                variables: &program.variables,
                global_variables,
                lists: &program.lists,
                global_lists,
                procedures: &program.procedures,
                procedure: None,
            },
            &program.declrs,
            false,
        )?;
        self.end_object()?;
        self.comma(true)?;
        self.key("costumes")?;
        self.begin_array()?;
        for declr in &program.declrs {
            let mut comma = false;
            if let Declr::Costumes(costumes, _span) = &*declr.borrow() {
                for (costume, span) in costumes {
                    self.costume(reports, costume.clone(), span, comma)?;
                    comma = true;
                }
            }
        }
        self.end_array()?;
        self.comma(true)?;
        self.key("variables")?;
        self.begin_object()?;
        let mut comma = false;
        for variable in &program.variables {
            if global_variables.is_some_and(|it| it.contains(variable)) {
                continue;
            }
            self.comma(comma)?;
            write!(self, r#"{}:[{},0]"#, json!(variable), json!((variable)))?;
            comma = true;
        }
        self.end_object()?;
        self.comma(true)?;
        self.key("sounds")?;
        self.begin_array()?;
        self.end_array()?;
        self.end_object()?;
        Ok(())
    }

    fn declrs(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        declrs: &Declrs<'src>,
        mut comma: bool,
    ) -> io::Result<()> {
        for declr in declrs {
            if matches!(*declr.borrow(), Declr::Costumes(..) | Declr::Sounds(..)) {
                continue;
            }
            self.declr(r, sc, &declr.borrow(), comma)?;
            comma = true;
        }
        Ok(())
    }

    fn declr(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        declr: &Declr<'src>,
        comma: bool,
    ) -> io::Result<()> {
        match declr {
            Declr::Costumes(..) => unreachable!(),
            Declr::Sounds(..) => unreachable!(),
            Declr::Def(procedure) => self.def(r, sc, procedure, comma),
            Declr::OnFlag(body, span) => self.on_flag(r, sc, body, span, comma),
            Declr::OnKey(key, body, span) => {
                self.on_key(r, sc, key.clone(), body, span.clone(), comma)
            }
            Declr::OnClick(body, span) => self.on_click(r, sc, body, span, comma),
            Declr::OnBackdrop(backdrop, body, span) => {
                self.on_backdrop(r, sc, backdrop, body, span, comma)
            }
            Declr::OnLoudnessGreaterThan(loudness, body, span) => self
                .on_loudness_greater_than(r, sc, &loudness.borrow(), body, span, comma),
            Declr::OnTimerGreaterThan(timer, body, span) => {
                self.on_timer_greater_than(r, sc, &timer.borrow(), body, span, comma)
            }
            Declr::OnMessage(message, body, span) => {
                self.on_message(r, sc, message, body, span, comma)
            }
            Declr::OnClone(body, span) => self.on_clone(r, sc, body, span, comma),
        }
    }

    fn costume(
        &mut self,
        r: R<'src, '_>,
        costume: Rc<str>,
        span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let (name, extension) = costume.rsplit_once('.').unwrap();
        if let Some(hash) = self.costumes.get(costume.as_ref()) {
            let name = name.replace("{{fwslash}}", "/");
            write!(self.zip, "{{")?;
            write!(self.zip, r#""name":{}"#, json!(name))?;
            write!(self.zip, r#","assetId":{}"#, json!(hash))?;
            write!(self.zip, r#","dataFormat":{}"#, json!(extension))?;
            write!(self.zip, r#","md5ext":{}"#, json!(format!("{hash}.{extension}")))?;
            write!(self.zip, "}}")?;
        } else {
            let path = self.input.join(costume.as_ref());
            let mut file = File::open(path)?;
            let mut hasher = Md5::new();
            io::copy(&mut file, &mut hasher)?;
            io::copy(&mut file, &mut hasher)?;
            let hash = hasher.finalize();
            self.costumes.insert(costume.clone(), format!("{:x}", hash));
            return self.costume(r, costume, span, comma);
        }
        Ok(())
    }

    fn sounds(
        &mut self,
        _r: R<'src, '_>,
        _sc: Sc<'src, '_>,
        _sounds: &[(String, Span)],
        _span: &Span,
        _comma: bool,
    ) -> io::Result<()> {
        todo!()
    }

    fn def(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        procedure: &Procedure<'src>,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let prototype_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("procedures_definition", this_id, comma)
                .top_level()
                .some_next_id((!procedure.body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.key("custom_block")?;
        write!(self, r#"[1,{}]"#, prototype_id)?;
        self.end_object()?;
        self.end_object()?;

        let mut arg_ids = Vec::new();
        for (arg, _) in &procedure.args {
            let arg_id = self.id.create_id();
            arg_ids.push(arg_id);
            self.begin_node(
                Node::new("argument_reporter_string_number", arg_id, true)
                    .parent_id(prototype_id)
                    .shadow(),
            )?;
            self.comma(true)?;
            self.key("fields")?;
            self.begin_object()?;
            self.key("VALUE")?;
            write!(self, r#"[{},null]"#, json!(arg))?;
            self.end_object()?;
            self.end_object()?;
        }

        self.begin_node(
            Node::new("procedures_prototype", prototype_id, true)
                .parent_id(this_id)
                .shadow(),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        let mut comma = false;
        for ((arg, _), arg_id) in procedure.args.iter().zip(arg_ids) {
            self.comma(comma)?;
            write!(self, r#"{}:[2,{}]"#, json!(arg), arg_id)?;
            comma = true;
        }
        self.end_object()?;
        self.begin_mutation_object()?;
        self.proccode(procedure.name, procedure.args.len())?;
        self.argument_stuff(&procedure.args)?;
        write!(self.zip, r#","argumentdefaults":"["#)?;
        let mut comma = false;
        for _ in &procedure.args {
            self.comma(comma)?;
            write!(self.zip, r#"\"\""#)?;
            comma = true;
        }
        write!(self.zip, r#"]""#)?;
        self.warp(procedure.warp)?;
        self.end_object()?;
        self.end_object()?;

        self.stmts(
            r,
            Sc {
                variables: sc.variables,
                global_variables: sc.global_variables,
                lists: sc.lists,
                global_lists: sc.global_lists,
                procedures: sc.procedures,
                procedure: sc.procedures.get(procedure.name),
            },
            &procedure.body,
            next_id,
            Some(this_id),
            true,
        )
    }

    fn begin_mutation_object(&mut self) -> Result<(), io::Error> {
        write!(self, r#","mutation":{{"tagName":"mutation","children":[]"#)
    }

    fn warp(&mut self, warp: bool) -> io::Result<()> {
        if warp {
            write!(self.zip, r#","warp":"true""#)
        } else {
            write!(self.zip, r#","warp":"false""#)
        }
    }

    fn argument_stuff(&mut self, args: &Names) -> io::Result<()> {
        for each in ["argumentids", "argumentnames"] {
            write!(self.zip, r#","{}":"["#, each)?;
            let mut comma = false;
            for (arg, _) in args {
                self.comma(comma)?;
                write!(self.zip, "{}", json!(arg).to_string().replace('"', r#"\""#))?;
                comma = true;
            }
            write!(self.zip, r#"]""#)?;
        }
        Ok(())
    }

    fn proccode(&mut self, name: &str, args: usize) -> io::Result<()> {
        write!(self, r#","proccode":"{}"#, name)?;
        for _ in 0..args {
            write!(self, r#" %s"#)?;
        }
        write!(self.zip, r#"""#)
    }

    fn on_flag(
        &mut self,
        r: &mut Vec<Report<'src>>,
        sc: Sc<'src, '_>,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whenflagclicked", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_key(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        key: Rc<str>,
        body: &Stmts<'src>,
        span: Span,
        comma: bool,
    ) -> io::Result<()> {
        if !KEYS.contains(&key.as_ref()) {
            r.push(Report::UnknownKey(key.clone(), span.clone()));
        }
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whenkeypressed", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("KEY_OPTION")?;
        write!(self, r#"[{},null]"#, json!(key.as_ref()))?;
        self.end_object()?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_click(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whenthisspriteclicked", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_backdrop(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        backdrop: &str,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whenbackdropswitchesto", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("BACKDROP_OPTION")?;
        write!(self, r#"[{},null]"#, json!(backdrop))?;
        self.end_object()?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_loudness_greater_than(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        loudness: &Expr<'src>,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let loudness_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whengreaterthan", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "VALUE", loudness, loudness_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("WHENGREATERTHANMENU")?;
        write!(self, r#"["LOUDNESS",null]"#)?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, loudness, loudness_id, this_id, true)?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_timer_greater_than(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        timer: &Expr<'src>,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let timer_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whengreaterthan", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "VALUE", timer, timer_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("WHENGREATERTHANMENU")?;
        write!(self, r#"["TIMER",null]"#)?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, timer, timer_id, this_id, true)?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_message(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        message: &str,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("event_whenbroadcastreceived", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("BROADCAST_OPTION")?;
        write!(self, r#"[{},null]"#, json!(message))?;
        self.end_object()?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn on_clone(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        body: &Stmts<'src>,
        _span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("control_start_as_clone", this_id, comma)
                .top_level()
                .some_next_id((!body.is_empty()).then_some(next_id)),
        )?;
        self.end_object()?;
        self.stmts(r, sc, body, next_id, Some(this_id), true)
    }

    fn stmts(
        &mut self,
        r: &mut Vec<Report<'src>>,
        sc: Sc<'src, '_>,
        stmts: &Stmts<'src>,
        mut this_id: BlockID,
        mut parent_id: Option<BlockID>,
        mut comma: bool,
    ) -> io::Result<()> {
        for (i, stmt) in stmts.iter().enumerate() {
            let is_before_unreachable_code = matches!(
                &*stmt.borrow(),
                Stmt::Block(
                    Block::StopThisScript | Block::StopAll | Block::DeleteThisClone,
                    ..
                )
            );
            if i == stmts.len() - 1 || is_before_unreachable_code {
                self.stmt(r, sc, &stmt.borrow(), this_id, None, parent_id, comma)?;
                if is_before_unreachable_code {
                    if i != stmts.len() - 1 {
                        if let Stmt::Block(_, _, span) = &*stmt.borrow() {
                            r.push(Report::UnreachableCode(span.clone()));
                        } else {
                            unreachable!()
                        }
                    }
                    break;
                }
            } else {
                let next_id = self.id.create_id();
                self.stmt(
                    r,
                    sc,
                    &stmt.borrow(),
                    this_id,
                    Some(next_id),
                    parent_id,
                    comma,
                )?;
                parent_id = Some(this_id);
                this_id = next_id;
            }
            comma = true;
        }
        Ok(())
    }

    fn stmt(
        &mut self,
        r: &mut Vec<Report<'src>>,
        sc: Sc<'src, '_>,
        stmt: &Stmt<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        match stmt {
            Stmt::Repeat(times, body, span) => self.repeat(
                r,
                sc,
                &times.borrow(),
                body,
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::Forever(body, span) => {
                self.forever(r, sc, body, span, this_id, next_id, parent_id, comma)
            }
            Stmt::Branch(condition, if_body, else_body, span) => self.branch(
                r,
                sc,
                &condition.borrow(),
                if_body,
                else_body,
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::Until(condition, body, span) => self.until(
                r,
                sc,
                &condition.borrow(),
                body,
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::SetVariable(name, expr, span) => self.set_variable(
                r,
                sc,
                name,
                &expr.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::ChangeVariable(name, expr, span) => self.change_variable(
                r,
                sc,
                name,
                &expr.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::Show(name, span) => {
                self.show(r, sc, name, span, this_id, next_id, parent_id, comma)
            }
            Stmt::Hide(name, span) => {
                self.hide(r, sc, name, span, this_id, next_id, parent_id, comma)
            }
            Stmt::ListAdd(name, expr, span) => self.list_add(
                r,
                sc,
                name,
                &expr.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::ListDelete(name, index, span) => self.list_delete(
                r,
                sc,
                name,
                &index.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::ListDeleteAll(name, span) => self
                .list_delete_all(r, sc, name, span, this_id, next_id, parent_id, comma),
            Stmt::ListInsert(name, index, expr, span) => self.list_insert(
                r,
                sc,
                name,
                &index.borrow(),
                &expr.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::ListReplace(name, index, expr, span) => self.list_replace(
                r,
                sc,
                name,
                &index.borrow(),
                &expr.borrow(),
                span,
                this_id,
                next_id,
                parent_id,
                comma,
            ),
            Stmt::Block(block, args, span) => {
                self.block(r, sc, block, args, span, this_id, next_id, parent_id, comma)
            }
            Stmt::ProcedureCall(name, args, span) => {
                self.call(r, sc, name, args, span, this_id, next_id, parent_id, comma)
            }
        }
    }

    fn substack(
        &mut self,
        comma: bool,
        name: &'static str,
        id: Option<BlockID>,
    ) -> io::Result<()> {
        if let Some(id) = id {
            self.comma(comma)?;
            write!(self, r#""{name}":[2,{}]"#, id)?;
        }
        Ok(())
    }

    fn repeat(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        times: &Expr<'src>,
        body: &Stmts<'src>,
        _span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let times_id = self.id.create_id();
        let body_id = self.id.create_id();
        self.begin_node(
            Node::new("control_repeat", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "TIMES", times, times_id, false)?;
        self.substack(true, "SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, times, times_id, this_id, true)?;
        self.stmts(r, sc, body, body_id, Some(this_id), true)
    }

    fn forever(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        body: &Stmts<'src>,
        _span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let body_id = self.id.create_id();
        self.begin_node(
            Node::new("control_forever", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.substack(false, "SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_object()?;
        self.end_object()?;
        self.stmts(r, sc, body, body_id, Some(this_id), true)
    }

    fn branch(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        condition: &Expr<'src>,
        if_body: &Stmts<'src>,
        else_body: &Stmts<'src>,
        _span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let condition_id = self.id.create_id();
        let if_id = self.id.create_id();
        let else_id = self.id.create_id();
        self.begin_node(
            Node::new(
                if else_body.is_empty() { "control_if" } else { "control_if_else" },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "CONDITION", condition, condition_id, false)?;
        self.substack(true, "SUBSTACK", (!if_body.is_empty()).then_some(if_id))?;
        self.substack(true, "SUBSTACK2", (!else_body.is_empty()).then_some(else_id))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, condition, condition_id, this_id, true)?;
        self.stmts(r, sc, if_body, if_id, Some(this_id), true)?;
        self.stmts(r, sc, else_body, else_id, Some(this_id), true)
    }

    fn until(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        condition: &Expr<'src>,
        body: &Stmts<'src>,
        _span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let condition_id = self.id.create_id();
        let body_id = self.id.create_id();
        self.begin_node(
            Node::new("control_repeat_until", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "CONDITION", condition, condition_id, false)?;
        self.substack(true, "SUBSTACK", (!body.is_empty()).then_some(body_id))?;
        self.end_object()?;
        self.end_object()?;
        self.stmts(r, sc, body, body_id, Some(this_id), true)
    }

    /// Report an error if name is not a variable.
    fn variable(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
    ) {
        if sc.variables.contains(name) {
            return;
        }
        if sc.global_variables.is_some_and(|it| it.contains(name)) {
            return;
        }
        r.push(Report::UndefinedVariable(name, span.clone()));
    }

    /// Report an error if name is not a list.
    fn list(&mut self, r: R<'src, '_>, sc: Sc<'src, '_>, name: &'src str, span: &Span) {
        if sc.lists.contains(name) {
            return;
        }
        if sc.global_lists.is_some_and(|it| it.contains(name)) {
            return;
        }
        r.push(Report::UndefinedList(name, span.clone()));
    }

    /// Return true if name is a variable, false if is a list or undefined.
    /// If name is undefined, report an error.
    fn variable_or_list(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
    ) -> bool {
        if sc.variables.contains(name) {
            return true;
        }
        if sc.global_variables.is_some_and(|it| it.contains(name)) {
            return true;
        }
        if sc.lists.contains(name) {
            return false;
        }
        if sc.global_lists.is_some_and(|it| it.contains(name)) {
            return false;
        }
        r.push(Report::UndefinedVariable(name, span.clone()));
        false
    }

    fn set_variable(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        expr: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.variable(r, sc, name, span);
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new("data_setvariableto", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "VALUE", expr, expr_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("VARIABLE")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn change_variable(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        expr: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.variable(r, sc, name, span);
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new("data_changevariableby", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "VALUE", expr, expr_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("VARIABLE")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn show(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_variable = self.variable_or_list(r, sc, name, span);
        self.begin_node(
            Node::new(
                if is_variable { "data_showvariable" } else { "data_showlist" },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key(if is_variable { "VARIABLE" } else { "LIST" })?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()
    }

    fn hide(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_variable = self.variable_or_list(r, sc, name, span);
        self.begin_node(
            Node::new(
                if is_variable { "data_hidevariable" } else { "data_hidelist" },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key(if is_variable { "VARIABLE" } else { "LIST" })?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()
    }

    fn list_add(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        expr: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new("data_addtolist", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "ITEM", expr, expr_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn list_delete(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        index: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        let index_id = self.id.create_id();
        self.begin_node(
            Node::new("data_deleteoflist", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "INDEX", index, index_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, index, index_id, this_id, true)
    }

    fn list_delete_all(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        self.begin_node(
            Node::new("data_deletealloflist", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()
    }

    fn list_insert(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        index: &Expr<'src>,
        expr: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        let index_id = self.id.create_id();
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new("data_insertatlist", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "INDEX", index, index_id, false)?;
        self.input(r, sc, "ITEM", expr, expr_id, true)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, index, index_id, this_id, true)?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn list_replace(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        index: &Expr<'src>,
        expr: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        let index_id = self.id.create_id();
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new("data_replaceitemoflist", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "INDEX", index, index_id, false)?;
        self.input(r, sc, "ITEM", expr, expr_id, true)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, index, index_id, this_id, true)?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn menu(
        &mut self,
        opcode: &'static str,
        name: &str,
        value: &str,
        this_id: BlockID,
        parent_id: BlockID,
    ) -> io::Result<()> {
        self.begin_node(
            Node::new(opcode, this_id, true).parent_id(parent_id).shadow(),
        )?;
        write!(self, r#","fields":{{"{name}":["{value}",null]}}}}"#)?;
        Ok(())
    }

    fn block(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        match block {
            | Block::GotoRandomPosition
            | Block::GotoMousePointer
            | Block::GotoSprite
            | Block::PointTowardsRandomDirection
            | Block::PointTowardsMousePointer
            | Block::PointTowards => {
                return self.goto_sprite_or_point_towards(
                    r, sc, block, args, this_id, next_id, parent_id, comma,
                )
            }
            | Block::GlideToRandomPosition
            | Block::GlideToMousePointer
            | Block::GlideToSprite => {
                return self.glide_to_sprite(
                    r, sc, block, args, this_id, next_id, parent_id, comma,
                )
            }
            Block::SwitchCostume | Block::SwitchBackdrop => {
                return self.switch_costume_or_backdrop(
                    r, sc, block, args, this_id, next_id, parent_id, comma,
                )
            }
            Block::PlaySoundUntilDone | Block::StartSound => {
                return self.play_sound_until_done_or_start_sound(
                    r, sc, block, args, this_id, next_id, parent_id, comma,
                )
            }
            Block::Clone | Block::CloneSprite => {
                return self
                    .clone(r, sc, block, args, this_id, next_id, parent_id, comma)
            }
            | Block::SetPenHue
            | Block::SetPenSaturation
            | Block::SetPenBrightness
            | Block::SetPenTransparency
            | Block::ChangePenHue
            | Block::ChangePenSaturation
            | Block::ChangePenBrightness
            | Block::ChangePenTransparency => {
                return self
                    .pen_param(r, sc, block, args, this_id, next_id, parent_id, comma)
            }
            Block::Breakpoint | Block::Log | Block::Warn | Block::Error => {
                return self.sa_debugger(
                    r, sc, block, args, this_id, next_id, parent_id, comma,
                )
            }
            _ => {}
        }
        let (opcode, arg_names, fields) = block_details(*block);
        match args.len().cmp(&arg_names.len()) {
            Ordering::Less => {
                r.push(Report::TooFewArgsForBlock {
                    block: *block,
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Greater => {
                r.push(Report::TooManyArgsForBlock {
                    block: *block,
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Equal => {}
        }
        let mut ids = Vec::with_capacity(arg_names.len());
        self.begin_node(
            Node::new(opcode, this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        let mut comma = false;
        for (&name, arg) in arg_names.iter().zip(args) {
            let id = self.id.create_id();
            ids.push(id);
            self.input(r, sc, name, &arg.borrow(), id, comma)?;
            comma = true;
        }
        self.end_object()?;
        if let Some(fields) = fields {
            self.comma(true)?;
            self.key("fields")?;
            self.begin_object()?;
            self.write_all(fields.as_bytes())?;
            self.end_object()?;
        }
        if matches!(block, Block::StopOtherScripts) {
            write!(
                self,
                r#","mutation":{{"tagName":"mutation","children":[],"hasnext": "true"}}"#
            )?;
        }
        self.end_object()?;
        for (arg, id) in args.iter().zip(ids) {
            self.expr(r, sc, &arg.borrow(), id, this_id, true)?;
        }
        Ok(())
    }

    // TODO: add errors for non-existant sprite references
    fn goto_sprite_or_point_towards(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_towards = matches!(
            block,
            Block::PointTowardsRandomDirection
                | Block::PointTowardsMousePointer
                | Block::PointTowards
        );
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new(
                if is_towards { "motion_pointtowards" } else { "motion_goto" },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() && args.len() == 1 {
            self.input_with_shadow(
                r,
                sc,
                if is_towards { "TOWARDS" } else { "TO" },
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
            self.end_object()?;
            self.end_object()?;
            self.expr(r, sc, &args[0].borrow(), arg_id, this_id, comma)?;
        } else {
            self.key(if is_towards { "TOWARDS" } else { "TO" })?;
            write!(self, r#"[1,{}]"#, menu_id)?;
            self.end_object()?;
            self.end_object()?;
        }
        self.menu(
            if is_towards { "motion_pointtowards_menu" } else { "motion_goto_menu" },
            if is_towards { "TOWARDS" } else { "TO" },
            match block {
                Block::GotoRandomPosition | Block::PointTowardsRandomDirection => {
                    "_random_"
                }
                Block::GotoMousePointer | Block::PointTowardsMousePointer => "_mouse_",
                Block::GotoSprite | Block::PointTowards => {
                    if let Some(literal) = literal {
                        literal
                    } else {
                        "_random_"
                    }
                }
                _ => unreachable!(),
            },
            menu_id,
            this_id,
        )
    }

    // TODO: add errors for non-existant sprite references
    fn glide_to_sprite(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> Result<(), io::Error> {
        let arg_id = self.id.create_id();
        let secs_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new("motion_glideto", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() && args.len() == 2 {
            self.input_with_shadow(
                r,
                sc,
                "TO",
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key("TO")?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.input(r, sc, "SECS", &args.last().unwrap().borrow(), secs_id, true)?;
        self.end_object()?;
        self.end_object()?;
        self.menu(
            "motion_glideto_menu",
            "TO",
            match block {
                Block::GlideToRandomPosition => "_random_",
                Block::GlideToMousePointer => "_mouse_",
                Block::GlideToSprite => {
                    if let Some(literal) = literal {
                        literal
                    } else {
                        "_random_"
                    }
                }
                _ => unreachable!(),
            },
            menu_id,
            this_id,
        )?;
        if args.len() == 2 {
            self.expr(r, sc, &args[1].borrow(), secs_id, this_id, true)?;
        }
        self.expr(r, sc, &args[0].borrow(), arg_id, this_id, true)
    }

    // TODO: add errors for non-existant sprite references
    fn clone(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> Result<(), io::Error> {
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new("control_create_clone_of", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() && args.len() == 1 {
            self.input_with_shadow(
                r,
                sc,
                "CLONE_OPTION",
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key("CLONE_OPTION")?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.end_object()?;
        self.end_object()?;
        if args.len() == 1 {
            self.expr(r, sc, &args[0].borrow(), arg_id, this_id, true)?;
        }
        self.menu(
            "control_create_clone_of_menu",
            "CLONE_OPTION",
            if let Some(literal) = literal { literal } else { "_myself_" },
            menu_id,
            this_id,
        )
    }

    // TODO: Add a way to put a expr in the menu.
    fn pen_param(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_change = matches!(
            block,
            Block::ChangePenHue
                | Block::ChangePenSaturation
                | Block::ChangePenBrightness
                | Block::ChangePenTransparency
        );
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        self.begin_node(
            Node::new(
                if is_change {
                    "pen_setPenColorParamTo"
                } else {
                    "pen_changePenColorParamBy"
                },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.key("COLOR_PARAM")?;
        write!(self, r#"[1,{menu_id}]"#)?;
        self.input(r, sc, "VALUE", &args[0].borrow(), arg_id, true)?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, &args[0].borrow(), this_id, this_id, true)?;
        self.menu(
            "pen_menu_colorParam",
            "colorParam",
            match block {
                Block::SetPenHue | Block::ChangePenHue => "color",
                Block::SetPenSaturation | Block::ChangePenSaturation => "saturation",
                Block::SetPenBrightness | Block::ChangePenBrightness => "brightness",
                Block::SetPenTransparency | Block::ChangePenTransparency => {
                    "transparency"
                }
                _ => unreachable!(),
            },
            menu_id,
            this_id,
        )
    }

    fn switch_costume_or_backdrop(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_costume = matches!(block, Block::SwitchCostume);
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new(
                if is_costume {
                    "looks_switchcostumeto"
                } else {
                    "looks_switchbackdropto"
                },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() {
            self.input_with_shadow(
                r,
                sc,
                if is_costume { "COSTUME" } else { "BACKDROP" },
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key(if is_costume { "COSTUME" } else { "BACKDROP" })?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.end_object()?;
        self.end_object()?;
        self.menu(
            if is_costume { "looks_costume" } else { "looks_backdrops" },
            if is_costume { "COSTUME" } else { "BACKDROP" },
            literal.map_or("make gh issue if this bothers u", |it| it.as_str()),
            menu_id,
            this_id,
        )
    }

    fn play_sound_until_done_or_start_sound(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_start_sound = matches!(block, Block::StartSound);
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new(
                if is_start_sound { "sound_play" } else { "sound_playuntildone" },
                this_id,
                comma,
            )
            .some_next_id(next_id)
            .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() {
            self.input_with_shadow(
                r,
                sc,
                "SOUND_MENU",
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key("SOUND_MENU")?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.end_object()?;
        self.end_object()?;
        self.menu(
            "sound_sounds_menu",
            "SOUND_MENU",
            literal.map_or("make gh issue if this bothers u", |it| it.as_str()),
            menu_id,
            this_id,
        )
    }

    fn call(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        args: &Exprs<'src>,
        span: &Span,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let Some(procedure) = sc.procedures.get(name) else {
            r.push(Report::UndefinedBlock(name, span.clone()));
            return Ok(());
        };
        match args.len().cmp(&procedure.args.len()) {
            Ordering::Less => {
                r.push(Report::TooFewArgsForProcedure {
                    procedure: procedure.clone(),
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Greater => {
                r.push(Report::TooManyArgsForProcedure {
                    procedure: procedure.clone(),
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Equal => {}
        }
        let mut ids = Vec::with_capacity(procedure.args.len());
        self.begin_node(
            Node::new("procedures_call", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        let mut comma = false;
        for ((name, _), arg) in procedure.args.iter().zip(args) {
            let id = self.id.create_id();
            ids.push(id);
            self.input(r, sc, name, &arg.borrow(), id, comma)?;
            comma = true;
        }
        self.end_object()?;
        self.begin_mutation_object()?;
        self.proccode(name, procedure.args.len())?;
        self.argument_stuff(&procedure.args)?;
        self.warp(procedure.warp)?;
        self.end_object()?;
        self.end_object()?;
        for (arg, id) in args.iter().zip(ids) {
            self.expr(r, sc, &arg.borrow(), id, this_id, true)?;
        }
        Ok(())
    }

    fn sa_debugger(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        block: &Block,
        args: &Exprs<'src>,
        this_id: BlockID,
        next_id: Option<BlockID>,
        parent_id: Option<BlockID>,
        comma: bool,
    ) -> io::Result<()> {
        let is_breakpoint = matches!(block, Block::Breakpoint);
        let arg_id = self.id.create_id();
        self.begin_node(
            Node::new("procedures_call", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if !is_breakpoint {
            self.input(r, sc, "arg0", &args[0].borrow(), arg_id, false)?;
        }
        self.end_object()?;
        self.begin_mutation_object()?;
        self.proccode(
            match block {
                Block::Breakpoint => "\\u200b\\u200bbreakpoint\\u200b\\u200b",
                Block::Warn => "\\u200b\\u200bwarn\\u200b\\u200b",
                Block::Error => "\\u200b\\u200berror\\u200b\\u200b",
                Block::Log => "\\u200b\\u200blog\\u200b\\u200b",
                _ => unreachable!(),
            },
            if is_breakpoint { 0 } else { 1 },
        )?;
        if is_breakpoint {
            self.argument_stuff(&vec![])?;
        } else {
            self.argument_stuff(&vec![("arg0", 0..0)])?;
        }

        self.warp(false)?;
        self.end_object()?;
        self.end_object()?;
        if !is_breakpoint {
            self.expr(r, sc, &args[0].borrow(), arg_id, this_id, true)?;
        }
        Ok(())
    }

    fn expr(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        expr: &Expr<'src>,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        match expr {
            Expr::Int(_, _) => Ok(()),
            Expr::Float(_, _) => Ok(()),
            Expr::String(_, _) => Ok(()),
            Expr::Name(_, _) => Ok(()),
            Expr::Arg(name, span) => {
                self.arg(r, sc, name, span, this_id, parent_id, comma)
            }
            Expr::Reporter(reporter, args, span) => {
                self.reporter(r, sc, reporter, args, span, this_id, parent_id, comma)
            }
            Expr::UnaryOp(op, expr, span) => self.unary_op(
                r,
                sc,
                op,
                &expr.borrow(),
                span,
                this_id,
                parent_id,
                comma,
            ),
            Expr::BinaryOp(op, left, right, span) => self.binary_op(
                r,
                sc,
                op,
                &left.borrow(),
                &right.borrow(),
                span,
                this_id,
                parent_id,
                comma,
            ),
        }
    }

    fn arg(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        if !(sc.procedure.is_some_and(|it| it.args_set.contains(name))) {
            r.push(Report::UndefinedArg {
                procedure: sc.procedure.unwrap().clone(),
                name,
                span: span.clone(),
            });
        }
        self.begin_node(
            Node::new("argument_reporter_string_number", this_id, comma)
                .parent_id(parent_id),
        )?;
        write!(self, r#","fields":{{"VALUE":[{}, null]}}}}"#, json!(name))
    }

    fn reporter(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        reporter: &Reporter,
        args: &Vec<Rrc<Expr<'src>>>,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        match reporter {
            | Reporter::TouchingMousePointer
            | Reporter::TouchingEdge
            | Reporter::Touching => {
                return self.touching_object(
                    r, sc, reporter, args, span, this_id, parent_id, comma,
                )
            }
            Reporter::DistanceToMousePointer | Reporter::DistanceTo => {
                return self.distance_to_object(
                    r, sc, reporter, args, span, this_id, parent_id, comma,
                )
            }
            _ => {}
        }
        let (opcode, arg_names, fields) = reporter_details(*reporter);
        match args.len().cmp(&arg_names.len()) {
            Ordering::Less => {
                r.push(Report::TooFewArgsForReporter {
                    reporter: *reporter,
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Greater => {
                r.push(Report::TooManyArgsForReporter {
                    reporter: *reporter,
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Equal => {}
        }
        let mut ids = Vec::with_capacity(arg_names.len());
        self.begin_node(Node::new(opcode, this_id, comma).parent_id(parent_id))?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        let mut comma = false;
        for (&name, arg) in arg_names.iter().zip(args) {
            let id = self.id.create_id();
            ids.push(id);
            self.input(r, sc, name, &arg.borrow(), id, comma)?;
            comma = true;
        }
        self.end_object()?;
        if let Some(fields) = fields {
            self.comma(true)?;
            self.key("fields")?;
            self.begin_object()?;
            self.write_all(fields.as_bytes())?;
            self.end_object()?;
        }
        self.end_object()?;
        for (arg, id) in args.iter().zip(ids) {
            self.expr(r, sc, &arg.borrow(), id, this_id, true)?;
        }
        Ok(())
    }

    fn touching_object(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        reporter: &Reporter,
        args: &Exprs<'src>,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new("sensing_touchingobject", this_id, comma).parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() && args.len() == 1 {
            self.input_with_shadow(
                r,
                sc,
                "TOUCHINGOBJECTMENU",
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key("TOUCHINGOBJECTMENU")?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.end_object()?;
        self.end_object()?;
        self.menu(
            "sensing_touchingobjectmenu",
            "TOUCHINGOBJECTMENU",
            match reporter {
                Reporter::TouchingMousePointer => "_mouse_",
                Reporter::TouchingEdge => "_edge_",
                Reporter::Touching => literal.map_or("_mouse_", |it| it.as_str()),
                _ => unreachable!(),
            },
            menu_id,
            this_id,
        )
    }

    fn distance_to_object(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        reporter: &Reporter,
        args: &Exprs<'src>,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        let arg_id = self.id.create_id();
        let menu_id = self.id.create_id();
        let literal = args.first().and_then(|arg| arg.borrow().literal_as_string());
        let literal = literal.as_ref();
        self.begin_node(
            Node::new("sensing_distanceto", this_id, comma).parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        if literal.is_none() && args.len() == 1 {
            self.input_with_shadow(
                r,
                sc,
                "DISTANCETOMENU",
                &args[0].borrow(),
                arg_id,
                menu_id,
                false,
            )?;
        } else {
            self.key("DISTANCETOMENU")?;
            write!(self, r#"[1,{}]"#, menu_id)?;
        }
        self.end_object()?;
        self.end_object()?;
        self.menu(
            "sensing_distancetomenu",
            "DISTANCETOMENU",
            match reporter {
                Reporter::DistanceToMousePointer => "_mouse_",
                Reporter::DistanceTo => literal.map_or("_mouse_", |it| it.as_str()),
                _ => unreachable!(),
            },
            menu_id,
            this_id,
        )?;
        if args.len() == 1 {
            self.expr(r, sc, &args[0].borrow(), arg_id, this_id, true)?;
        }
        Ok(())
    }

    fn unary_op(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        op: &UnaryOp,
        expr: &Expr<'src>,
        _span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        use UnaryOp as U;
        if matches!(op, U::Length) {
            if let Expr::Name(name, span) = expr {
                if sc.lists.contains(name)
                    || sc.global_lists.is_some_and(|it| it.contains(name))
                {
                    return self
                        .list_length(r, sc, name, span, this_id, parent_id, comma);
                }
            }
        }
        let expr_id = self.id.create_id();
        self.begin_node(
            Node::new(
                match op {
                    U::Minus => unreachable!(),
                    U::Not => "operator_not",
                    U::Length => "operator_length",
                    U::Round => "operator_round",
                    | U::Abs
                    | U::Floor
                    | U::Ceil
                    | U::Sqrt
                    | U::Sin
                    | U::Cos
                    | U::Tan
                    | U::Asin
                    | U::Acos
                    | U::Atan
                    | U::Ln
                    | U::Log
                    | U::AntiLn
                    | U::AntiLog => "operator_mathop",
                },
                this_id,
                comma,
            )
            .parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(
            r,
            sc,
            match op {
                U::Not => "OPERAND",
                U::Length => "STRING",
                _ => "NUM",
            },
            expr,
            expr_id,
            false,
        )?;
        self.end_object()?;
        if let Some(operator) = match op {
            U::Abs => Some("abs"),
            U::Floor => Some("floor"),
            U::Ceil => Some("ceiling"),
            U::Sqrt => Some("sqrt"),
            U::Sin => Some("sin"),
            U::Cos => Some("cos"),
            U::Tan => Some("tan"),
            U::Asin => Some("asin"),
            U::Acos => Some("acos"),
            U::Atan => Some("atan"),
            U::Ln => Some("ln"),
            U::Log => Some("log"),
            U::AntiLn => Some("e ^"),
            U::AntiLog => Some("10 ^"),
            _ => None,
        } {
            self.comma(true)?;
            self.key("fields")?;
            self.begin_object()?;
            self.key("OPERATOR")?;
            write!(self, r#"["{}",null]"#, operator)?;
            self.end_object()?;
        }
        self.end_object()?;
        self.expr(r, sc, expr, expr_id, this_id, true)
    }

    fn binary_op(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        op: &BinaryOp,
        left: &Expr<'src>,
        right: &Expr<'src>,
        _span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        use BinaryOp as B;
        if matches!(op, B::Of) {
            if let Expr::Name(name, span) = left {
                if sc.lists.contains(name)
                    || sc.global_lists.is_some_and(|it| it.contains(name))
                {
                    return self.list_item(
                        r, sc, name, right, span, this_id, parent_id, comma,
                    );
                }
            }
        }
        let left_id = self.id.create_id();
        let right_id = self.id.create_id();
        self.begin_node(
            Node::new(
                match op {
                    B::Add => "operator_add",
                    B::Sub => "operator_subtract",
                    B::Mul => "operator_multiply",
                    B::Div => "operator_divide",
                    B::Mod => "operator_mod",
                    B::Lt => "operator_lt",
                    B::Gt => "operator_gt",
                    B::Eq => "operator_equals",
                    B::And => "operator_and",
                    B::Or => "operator_or",
                    B::Join => "operator_join",
                    B::In => "operator_contains",
                    B::Of => "operator_letter_of",
                    B::Le | B::Ge | B::Ne => unreachable!(),
                },
                this_id,
                comma,
            )
            .parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(
            r,
            sc,
            match op {
                B::Join | B::In => "STRING1",
                B::Of => "STRING",
                B::Eq | B::Gt | B::Lt | B::And | B::Or => "OPERAND1",
                _ => "NUM1",
            },
            left,
            left_id,
            false,
        )?;
        self.input(
            r,
            sc,
            match op {
                B::Join | B::In => "STRING2",
                B::Of => "LETTER",
                B::Eq | B::Gt | B::Lt | B::And | B::Or => "OPERAND2",
                _ => "NUM2",
            },
            right,
            right_id,
            true,
        )?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, left, left_id, this_id, true)?;
        self.expr(r, sc, right, right_id, this_id, true)
    }

    fn list_item(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        index: &Expr<'src>,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        let index_id = self.id.create_id();
        self.begin_node(
            Node::new("data_itemoflist", this_id, comma).parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.input(r, sc, "INDEX", index, index_id, false)?;
        self.end_object()?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()?;
        self.expr(r, sc, index, index_id, this_id, true)
    }

    fn list_length(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &'src str,
        span: &Span,
        this_id: BlockID,
        parent_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        self.list(r, sc, name, span);
        self.begin_node(
            Node::new("data_lengthoflist", this_id, comma).parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("fields")?;
        self.begin_object()?;
        self.key("LIST")?;
        write!(self, r#"[{},null]"#, json!(name))?;
        self.end_object()?;
        self.end_object()
    }

    fn input(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &str,
        expr: &Expr<'src>,
        id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        self.comma(comma)?;
        self.key(name)?;
        self.input_literal(r, sc, name, expr, id, None)
    }

    fn input_with_shadow(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &str,
        expr: &Expr<'src>,
        id: BlockID,
        shadow_id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        self.comma(comma)?;
        self.key(name)?;
        self.input_literal(r, sc, name, expr, id, Some(shadow_id))
    }

    fn input_literal(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &str,
        expr: &Expr<'src>,
        id: BlockID,
        shadow_id: Option<BlockID>,
    ) -> io::Result<()> {
        match expr {
            Expr::Int(value, _span) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))
            }
            Expr::Float(value, _span) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))
            }
            Expr::String(value, _span) => {
                if name == "BROADCAST_INPUT" {
                    write!(
                        self,
                        r#"[1,[11,{},{}]]"#,
                        json!(value.as_ref()),
                        json!(value.as_ref())
                    )
                } else if name == "COLOR" || name == "COLOR2" {
                    write!(self, r#"[1,[9,{}]]"#, json!(value.as_ref()))
                } else {
                    write!(self, r#"[1,[10,{}]]"#, json!(value.as_ref()))
                }
            }
            _ => self.input_value(r, sc, name, expr, id, shadow_id),
        }
    }

    fn input_value(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        name: &str,
        expr: &Expr<'src>,
        id: BlockID,
        shadow_id: Option<BlockID>,
    ) -> io::Result<()> {
        match expr {
            Expr::Name(variable_name, span) => {
                if sc.variables.contains(variable_name)
                    || sc.global_variables.is_some_and(|it| it.contains(variable_name))
                {
                    write!(
                        self,
                        r#"[3,[12,{},{}],"#,
                        json!(variable_name),
                        json!(variable_name)
                    )?;
                } else if sc.lists.contains(variable_name)
                    || sc.global_lists.is_some_and(|it| it.contains(variable_name))
                {
                    write!(
                        self,
                        r#"[3,[13,{},{}],"#,
                        json!(variable_name),
                        json!(variable_name)
                    )?;
                } else {
                    r.push(Report::UndefinedVariable(variable_name, span.clone()));
                }
                if let Some(shadow_id) = shadow_id {
                    write!(self, r#"{shadow_id}"#)?;
                } else if name == "BROADCAST_INPUT" {
                    write!(self, r#"[11,"message1","message1"]"#)?;
                } else {
                    write!(self, r#"[10,""]"#)?;
                }
                self.end_array()
            }
            _ => {
                if name == "CONDITION" || name == "CONDITION2" {
                    write!(self, r#"[2,{id}]"#)
                } else {
                    write!(self, r#"[3,{id},"#)?;
                    if name == "BROADCAST_INPUT" {
                        write!(self, r#"[11,"message1","message1"]"#)?;
                    } else if let Some(shadow_id) = shadow_id {
                        write!(self, r#"{shadow_id}"#)?;
                    } else {
                        write!(self, r#"[10,""]"#)?;
                    }
                    self.end_array()
                }
            }
        }
    }
}

impl<T> io::Write for CodeGen<T>
where
    T: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.zip.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.zip.flush()
    }
}
