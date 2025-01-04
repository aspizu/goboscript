use logos::Span;

use super::Expr;
use crate::misc::SmolStr;

#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Box<Expr>,
}
