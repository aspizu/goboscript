use logos::Span;
use smol_str::SmolStr;

use super::Expr;

#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Box<Expr>,
}
