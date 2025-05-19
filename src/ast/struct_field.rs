use logos::Span;

use super::Value;
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
    pub default: Option<(Value, Span)>,
    pub is_used: bool,
}
