use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::*;
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Func {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub args: Vec<Arg>,
}

impl Func {
    pub fn new(name: SmolStr, span: Span, type_: Type, args: Vec<Arg>) -> Self {
        Self {
            name,
            span,
            type_,
            args,
        }
    }
}
