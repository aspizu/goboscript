use std::{cell::RefCell, path::Path, rc::Rc};

use fxhash::{FxHashMap, FxHashSet};
use logos::Span;
use serde::Serialize;
use smol_str::SmolStr;

use crate::blocks::{BinOp, Block, Repr, UnOp};

pub type Rrc<T> = Rc<RefCell<T>>;

#[derive(Debug, Default)]
pub struct Project {
    pub stage: Sprite,
    pub sprites: FxHashMap<SmolStr, Sprite>,
}

impl Project {
    pub fn new(stage: Sprite, mut sprites: FxHashMap<SmolStr, Sprite>) -> Self {
        for sprite in sprites.values_mut() {
            sprite.vars.retain(|name, _| !stage.vars.contains_key(name));
        }
        Self { stage, sprites }
    }
}

#[derive(Debug, Default)]
pub struct Sprite {
    pub costumes: Vec<Costume>,
    pub procs: FxHashMap<SmolStr, Proc>,
    pub used_procs: FxHashSet<SmolStr>,
    pub enums: FxHashMap<SmolStr, Enum>,
    pub vars: FxHashMap<SmolStr, Var>,
    pub lists: FxHashMap<SmolStr, List>,
    pub broadcasts: FxHashSet<SmolStr>,
    pub on_messages: FxHashMap<SmolStr, OnMessage>,
    pub events: Vec<Event>,
}

#[derive(Debug)]
pub struct Costume {
    pub name: SmolStr,
    pub path: SmolStr,
    pub span: Span,
}

impl Costume {
    pub fn new(path: SmolStr, span: Span, alias: Option<SmolStr>) -> Self {
        // TODO: validate file extension
        let name = alias.unwrap_or_else(|| {
            Path::new(path.as_str()).file_stem().unwrap().to_str().unwrap().into()
        });
        Self { name, path, span }
    }
}

#[derive(Debug)]
pub struct Enum {
    pub name: SmolStr,
    pub span: Span,
    pub variants: Vec<(SmolStr, Span)>,
    pub used_variants: FxHashSet<SmolStr>,
}

impl Enum {
    pub fn new(name: SmolStr, span: Span, variants: Vec<(SmolStr, Span)>) -> Self {
        Self { name, span, variants, used_variants: Default::default() }
    }
}

#[derive(Debug)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub default: Literal,
    pub used: bool,
}

impl Var {
    pub fn new(name: SmolStr, span: Span, default: Option<Literal>) -> Self {
        Self { name, span, default: default.unwrap_or(Literal::Int(0)), used: false }
    }
}

#[derive(Debug)]
pub struct List {
    pub name: SmolStr,
    pub span: Span,
    pub default: Literals,
    pub used: bool,
}

impl List {
    pub fn new(name: SmolStr, span: Span, default: Literals) -> Self {
        Self { name, span, default, used: false }
    }
}

#[derive(Debug, Default)]
pub struct References {
    pub procs: FxHashSet<SmolStr>,
    pub vars: FxHashSet<SmolStr>,
    pub lists: FxHashSet<SmolStr>,
    pub messages: FxHashSet<SmolStr>,
    pub enum_variants: FxHashSet<(SmolStr, SmolStr)>,
}

#[derive(Debug)]
pub struct Proc {
    pub name: SmolStr,
    pub span: Span,
    pub args: Vec<(SmolStr, Span)>,
    pub used_args: FxHashMap<SmolStr, bool>,
    pub locals: FxHashMap<SmolStr, Var>,
    pub body: Stmts,
    pub warp: bool,
    pub references: References,
}

impl Proc {
    pub fn new(
        name: SmolStr,
        span: Span,
        args: Vec<(SmolStr, Span)>,
        body: Stmts,
        warp: bool,
    ) -> Self {
        let used_args = args.iter().map(|(name, _)| (name.clone(), false)).collect();
        Self {
            name,
            span,
            args,
            used_args,
            body,
            warp,
            references: Default::default(),
            locals: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct OnMessage {
    pub message: SmolStr,
    pub span: Span,
    pub body: Stmts,
    pub used: bool,
    pub references: References,
}

impl OnMessage {
    pub fn new(message: SmolStr, span: Span, body: Stmts) -> Self {
        Self { message, span, body, used: false, references: Default::default() }
    }
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventDetail,
    pub span: Span,
    pub body: Stmts,
    pub references: References,
}

#[derive(Debug)]
pub enum EventDetail {
    OnFlag,
    OnKey { key: SmolStr, span: Span },
    OnClick,
    OnBackdrop { backdrop: SmolStr, span: Span },
    OnLoudnessGt { value: Rrc<Expr> },
    OnTimerGt { value: Rrc<Expr> },
    OnClone,
}

impl EventDetail {
    pub fn to_event(self, span: Span, body: Stmts) -> Event {
        Event { kind: self, span, body, references: Default::default() }
    }
}

pub type Stmts = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    Repeat {
        times: Rrc<Expr>,
        body: Stmts,
    },
    Forever {
        body: Stmts,
        span: Span,
    },
    Branch {
        cond: Rrc<Expr>,
        if_body: Stmts,
        else_body: Stmts,
    },
    Until {
        cond: Rrc<Expr>,
        body: Stmts,
    },
    SetVar {
        name: SmolStr,
        span: Span,
        value: Rrc<Expr>,
        is_local: bool,
    },
    ChangeVar {
        name: SmolStr,
        span: Span,
        value: Rrc<Expr>,
    },
    Show {
        name: SmolStr,
        span: Span,
    },
    Hide {
        name: SmolStr,
        span: Span,
    },
    ListAdd {
        name: SmolStr,
        span: Span,
        value: Rrc<Expr>,
    },
    ListDelete {
        name: SmolStr,
        span: Span,
        index: Rrc<Expr>,
    },
    ListDeleteAll {
        name: SmolStr,
        span: Span,
    },
    ListInsert {
        name: SmolStr,
        span: Span,
        index: Rrc<Expr>,
        value: Rrc<Expr>,
    },
    ListSet {
        name: SmolStr,
        span: Span,
        index: Rrc<Expr>,
        value: Rrc<Expr>,
    },
    ListChange {
        op: BinOp,
        name: SmolStr,
        span: Span,
        index: Rrc<Expr>,
        value: Rrc<Expr>,
    },
    Block {
        block: Block,
        span: Span,
        args: Exprs,
    },
    ProcCall {
        name: SmolStr,
        span: Span,
        args: Exprs,
    },
}

pub type Exprs = Vec<Rrc<Expr>>;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(SmolStr),
    Name {
        name: SmolStr,
        span: Span,
    },
    Arg {
        name: SmolStr,
        span: Span,
    },
    Repr {
        repr: Repr,
        span: Span,
        args: Exprs,
    },
    UnOp {
        op: UnOp,
        val: Rrc<Expr>,
    },
    BinOp {
        op: BinOp,
        lhs: Rrc<Expr>,
        rhs: Rrc<Expr>,
    },
    EnumVariant {
        enum_name: SmolStr,
        enum_span: Span,
        variant_name: SmolStr,
        variant_span: Span,
    },
}

impl Expr {
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Expr::Int(value) => Some(*value),
            Expr::Float(value) => Some(*value as i64),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.as_int().is_some_and(|it| it == 0)
    }

    pub fn try_to_string(&self) -> Option<String> {
        match self {
            Expr::Int(value) => Some(value.to_string()),
            Expr::Float(value) => Some(value.to_string()),
            Expr::Str(value) => Some(value.to_string()),
            _ => None,
        }
    }
}

impl From<Expr> for Rc<RefCell<Expr>> {
    fn from(val: Expr) -> Self {
        Rc::new(RefCell::new(val))
    }
}

pub type Literals = Vec<Literal>;

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(SmolStr),
}

impl Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            Literal::Int(value) => serializer.serialize_i64(*value),
            Literal::Float(value) => serializer.serialize_f64(*value),
            Literal::Str(value) => serializer.serialize_str(value),
        }
    }
}

impl UnOp {
    pub fn to_expr(self, val: Rrc<Expr>) -> Expr {
        Expr::UnOp { op: self, val }
    }
}

impl BinOp {
    pub fn to_expr(self, lhs: Rrc<Expr>, rhs: Rrc<Expr>) -> Expr {
        Expr::BinOp { op: self, lhs, rhs }
    }
}

impl Event {
    pub fn opcode(&self) -> &'static str {
        match &self.kind {
            EventDetail::OnFlag => "event_whenflagclicked",
            EventDetail::OnKey { .. } => "event_whenkeypressed",
            EventDetail::OnClick => "event_whenthisspriteclicked",
            EventDetail::OnBackdrop { .. } => "event_whenbackdropswitchesto",
            EventDetail::OnLoudnessGt { .. } | EventDetail::OnTimerGt { .. } => {
                "event_whengreaterthan"
            }
            EventDetail::OnClone => "control_start_as_clone",
        }
    }
}

impl Stmt {
    pub fn span(&self) -> &Span {
        match self {
            Stmt::Forever { span, .. } => span,
            Stmt::SetVar { span, .. } => span,
            Stmt::ChangeVar { span, .. } => span,
            Stmt::Show { span, .. } => span,
            Stmt::Hide { span, .. } => span,
            Stmt::ListAdd { span, .. } => span,
            Stmt::ListDelete { span, .. } => span,
            Stmt::ListDeleteAll { span, .. } => span,
            Stmt::ListInsert { span, .. } => span,
            Stmt::ListSet { span, .. } => span,
            Stmt::ListChange { span, .. } => span,
            Stmt::Block { span, .. } => span,
            Stmt::ProcCall { span, .. } => span,
            _ => unreachable!(),
        }
    }
}
