use logos::Span;
use serde::Serialize;

use super::{
    type_::Type,
    Expr,
};
use crate::misc::SmolStr;

#[derive(Debug, Serialize)]
pub struct Arg {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<Expr>,
}

impl Arg {
    pub fn new(name: SmolStr, span: Span, type_: Type, default: Option<Expr>) -> Self {
        Self {
            name,
            span,
            type_,
            default,
        }
    }
}
