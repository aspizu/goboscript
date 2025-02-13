use logos::Span;
use serde::Serialize;

use crate::misc::SmolStr;

#[derive(Debug, Serialize)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
    pub is_used: bool,
}
