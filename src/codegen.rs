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
        BinaryOp, Block, Declr, Declrs, Expr, Exprs, Function, Names, Reporter, Rrc,
        Stmt, Stmts, UnaryOp,
    },
    blockid::{BlockID, BlockIDFactory},
    build::{FunctionPrototype, Program},
    details::{block_details, reporter_details},
    reporting::Report,
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
}

type R<'src, 'b> = &'b mut Vec<Report<'src>>;

#[derive(Clone, Copy)]
pub struct Sc<'src, 'b> {
    variables: &'b HashSet<&'src str>,
    global_variables: Option<&'b HashSet<&'src str>>,
    lists: &'b HashSet<&'src str>,
    global_lists: Option<&'b HashSet<&'src str>>,
    functions: &'b HashMap<&'src str, FunctionPrototype<'src>>,
    function: Option<&'b FunctionPrototype<'src>>,
}

impl<'src, T> CodeGen<T>
where
    T: Write + Seek,
{
    pub fn new(zip: ZipFile<T>, input: PathBuf) -> CodeGen<T> {
        CodeGen { zip, id: Default::default(), costumes: Default::default(), input }
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
                functions: &program.functions,
                function: None,
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
            Declr::Def(function) => self.def(r, sc, function, comma),
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
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        sounds: &[(String, Span)],
        span: &Span,
        comma: bool,
    ) -> io::Result<()> {
        todo!()
    }

    fn def(
        &mut self,
        r: R<'src, '_>,
        sc: Sc<'src, '_>,
        function: &Function<'src>,
        comma: bool,
    ) -> io::Result<()> {
        let this_id = self.id.create_id();
        let prototype_id = self.id.create_id();
        let next_id = self.id.create_id();
        self.begin_node(
            Node::new("procedures_definition", this_id, comma)
                .top_level()
                .some_next_id((!function.body.is_empty()).then_some(next_id)),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        self.key("custom_block")?;
        write!(self, r#"[1,{}]"#, prototype_id)?;
        self.end_object()?;
        self.end_object()?;

        let mut arg_ids = Vec::new();
        for (arg, _) in &function.args {
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
        for ((arg, _), arg_id) in function.args.iter().zip(arg_ids) {
            self.comma(comma)?;
            write!(self, r#"{}:[2,{}]"#, json!(arg), arg_id)?;
            comma = true;
        }
        self.end_object()?;
        self.begin_mutation_object()?;
        self.proccode(function.name, function.args.len())?;
        self.argument_stuff(&function.args)?;
        write!(self.zip, r#","argumentdefaults":"["#)?;
        let mut comma = false;
        for _ in &function.args {
            self.comma(comma)?;
            write!(self.zip, r#"\"\""#)?;
            comma = true;
        }
        write!(self.zip, r#"]""#)?;
        self.warp(function.warp)?;
        self.end_object()?;
        self.end_object()?;

        self.stmts(
            r,
            Sc {
                variables: sc.variables,
                global_variables: sc.global_variables,
                lists: sc.lists,
                global_lists: sc.global_lists,
                functions: sc.functions,
                function: sc.functions.get(function.name),
            },
            &function.body,
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

    fn argument_stuff(&mut self, args: &Names<'src>) -> io::Result<()> {
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
        self.input("VALUE", loudness, loudness_id, false)?;
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
        self.input("VALUE", timer, timer_id, false)?;
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
            let is_stop_this_script =
                matches!(&*stmt.borrow(), Stmt::Block(Block::StopThisScript, ..));
            if i == stmts.len() - 1 || is_stop_this_script {
                self.stmt(r, sc, &stmt.borrow(), this_id, None, parent_id, comma)?;
                if is_stop_this_script {
                    match &*stmt.borrow() {
                        Stmt::Block(_, _, span) => {
                            r.push(Report::UnreachableCode(span.clone()));
                        }
                        _ => unreachable!(),
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
            Stmt::Call(name, args, span) => {
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
        self.input("TIMES", times, times_id, false)?;
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
        self.input("CONDITION", condition, condition_id, false)?;
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
        self.input("CONDITION", condition, condition_id, false)?;
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
        self.input("VALUE", expr, expr_id, false)?;
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
        self.input("VALUE", expr, expr_id, false)?;
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
        self.input("ITEM", expr, expr_id, false)?;
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
        self.input("INDEX", index, index_id, false)?;
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
        self.input("INDEX", index, index_id, false)?;
        self.input("ITEM", expr, expr_id, true)?;
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
        self.input("INDEX", index, index_id, false)?;
        self.input("ITEM", expr, expr_id, true)?;
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
            self.input(name, &arg.borrow(), id, comma)?;
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
        let Some(function) = sc.functions.get(name) else {
            r.push(Report::UndefinedBlock(name, span.clone()));
            return Ok(());
        };
        match args.len().cmp(&function.args.len()) {
            Ordering::Less => {
                r.push(Report::TooFewArgsForFunction {
                    function: function.clone(),
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Greater => {
                r.push(Report::TooManyArgsForFunction {
                    function: function.clone(),
                    given: args.len(),
                    span: span.clone(),
                });
            }
            Ordering::Equal => {}
        }
        let mut ids = Vec::with_capacity(function.args.len());
        self.begin_node(
            Node::new("procedures_call", this_id, comma)
                .some_next_id(next_id)
                .some_parent_id(parent_id),
        )?;
        self.comma(true)?;
        self.key("inputs")?;
        self.begin_object()?;
        let mut comma = false;
        for ((name, _), arg) in function.args.iter().zip(args) {
            let id = self.id.create_id();
            ids.push(id);
            self.input(name, &arg.borrow(), id, comma)?;
            comma = true;
        }
        self.end_object()?;
        self.begin_mutation_object()?;
        self.proccode(name, function.args.len())?;
        self.argument_stuff(&function.args)?;
        self.warp(function.warp)?;
        self.end_object()?;
        self.end_object()?;
        for (arg, id) in args.iter().zip(ids) {
            self.expr(r, sc, &arg.borrow(), id, this_id, true)?;
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
        if !(sc.function.is_some_and(|it| it.args_set.contains(name))) {
            r.push(Report::UndefinedArg(name, span.clone()));
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
            self.input(name, &arg.borrow(), id, comma)?;
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
        let expr_id = self.id.create_id();
        use UnaryOp as U;
        self.begin_node(
            Node::new(
                match op {
                    U::Minus => unreachable!(),
                    U::Not => "operator_not",
                    U::Length => "operator_length",
                    U::Round => "operator_round",
                    U::Abs
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

    fn input(
        &mut self,
        name: &str,
        expr: &Expr<'src>,
        id: BlockID,
        comma: bool,
    ) -> io::Result<()> {
        self.comma(comma)?;
        self.key(name)?;
        match expr {
            Expr::Int(value, _span) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))?;
            }
            Expr::Float(value, _span) => {
                write!(self, r#"[1,[4,{}]]"#, json!(value))?;
            }
            Expr::String(value, _span) => {
                if name == "COLOR" || name == "COLOR2" {
                    write!(self, r#"[1,[9,{}]]"#, json!(value.as_ref()))?;
                } else {
                    write!(self, r#"[1,[10,{}]]"#, json!(value.as_ref()))?;
                }
            }
            Expr::Name(name, _span) => {
                write!(self, r#"[3,[12,{},{}],[10,""]]"#, json!(name), json!(name))?;
            }
            _ => {
                if name == "CONDITION" || name == "CONDITION2" {
                    write!(self, r#"[2,{id}]"#)?;
                } else {
                    write!(self, r#"[3,{id},[10,""]]"#)?;
                }
            }
        }
        Ok(())
    }
}

impl<'src, T> io::Write for CodeGen<T>
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
