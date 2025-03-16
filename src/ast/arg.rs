use logos::Span;

use super::{
    type_::Type,
    ConstExpr,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Arg {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ConstExpr>,
}

impl Arg {
    pub fn new(name: SmolStr, span: Span, type_: Type, default: Option<ConstExpr>) -> Self {
        Self {
            name,
            span,
            type_,
            default,
        }
    }
}
