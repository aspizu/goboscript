use logos::Span;

use super::{
    type_::Type,
    Value,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Arg {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<(Value, Span)>,
}

impl Arg {
    pub fn new(name: SmolStr, span: Span, type_: Type, default: Option<(Value, Span)>) -> Self {
        Self {
            name,
            span,
            type_,
            default,
        }
    }
}
