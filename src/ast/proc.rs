use fxhash::FxHashMap;
use logos::Span;
use smol_str::SmolStr;

use super::{arg::Arg, stmt::Stmt, var::Var};

#[derive(Debug)]
pub struct Proc {
    pub name: SmolStr,
    pub span: Span,
    pub args: Vec<Arg>,
    pub locals: FxHashMap<SmolStr, Var>,
    pub body: Vec<Stmt>,
    pub warp: bool,
}

impl Proc {
    pub fn new(name: SmolStr, span: Span, args: Vec<Arg>, body: Vec<Stmt>, warp: bool) -> Self {
        Self {
            name,
            span,
            args,
            locals: FxHashMap::default(),
            body,
            warp,
        }
    }
}
