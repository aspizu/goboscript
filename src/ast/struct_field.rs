use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::ConstExpr;
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
    pub default: Option<ConstExpr>,
    pub is_used: bool,
}
