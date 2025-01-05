use fxhash::FxHashMap;
use logos::Span;

use super::*;
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Proc {
    pub name: SmolStr,
    pub span: Span,
    pub args: Vec<Arg>,
    pub locals: FxHashMap<SmolStr, Var>,
    pub warp: bool,
}

impl Proc {
    pub fn new(name: SmolStr, span: Span, args: Vec<Arg>, warp: bool) -> Self {
        Self {
            name,
            span,
            args,
            locals: FxHashMap::default(),
            warp,
        }
    }
}
