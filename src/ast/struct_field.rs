use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;
use super::ConstExpr;
use crate::misc::SmolStr;

#[derive(Tsify, Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
    pub default: Option<ConstExpr>,
    pub is_used: bool,
}
