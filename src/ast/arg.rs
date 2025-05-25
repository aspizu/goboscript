use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    type_::Type,
    Value,
};
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<(Value, Span)>,
    pub is_used: bool,
}

impl Arg {
    pub fn new(name: SmolStr, span: Span, type_: Type, default: Option<(Value, Span)>) -> Self {
        Self {
            name,
            span,
            type_,
            default,
            is_used: false,
        }
    }
}
