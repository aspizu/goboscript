use logos::Span;
use serde::{Deserialize, Serialize};

use super::Expr;
use crate::misc::SmolStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Box<Expr>,
}
