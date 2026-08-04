use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use super::Expr;
use crate::misc::SmolStr;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
pub struct StructLiteralField {
    pub name: SmolStr,
    pub span: Span,
    pub value: Box<Expr>,
}
