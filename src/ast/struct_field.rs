use logos::Span;

use crate::misc::SmolStr;

#[derive(Debug)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
    pub is_used: bool,
}
