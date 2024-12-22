use fxhash::FxHashMap;
use logos::Span;
use smol_str::SmolStr;

use super::{arg::Arg, stmt::Stmt, var::Var, Type};

#[derive(Debug)]
pub struct Func {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub args: Vec<Arg>,
    pub body: Vec<Stmt>,
    pub locals: FxHashMap<SmolStr, Var>,
}

impl Func {
    pub fn new(name: SmolStr, span: Span, type_: Type, args: Vec<Arg>, body: Vec<Stmt>) -> Self {
        Self {
            name,
            span,
            type_,
            args,
            body,
            locals: FxHashMap::default(),
        }
    }
}
