use logos::Span;
use serde::Serialize;

use super::Expr;
use crate::misc::SmolStr;

#[derive(Debug, Clone, Serialize)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Box<Expr>,
}
