use logos::Span;
use smol_str::SmolStr;

use super::Expr;
use crate::misc::Rrc;

#[derive(Debug)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Rrc<Expr>,
}
