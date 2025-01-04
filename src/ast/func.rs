use fxhash::FxHashMap;
use logos::Span;

use super::{arg::Arg, stmt::Stmt, var::Var, References, Type};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Func {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub args: Vec<Arg>,
    pub body: Vec<Stmt>,
    pub locals: FxHashMap<SmolStr, Var>,
    pub references: References,
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
            references: References::default(),
        }
    }
}
