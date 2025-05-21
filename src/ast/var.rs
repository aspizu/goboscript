use logos::Span;

use super::{
    type_::Type,
    Value,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<(Value, Span)>,
    pub is_cloud: bool,
    pub is_used: bool,
}
