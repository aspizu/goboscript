use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;
use super::{
    type_::Type,
    ConstExpr,
};
use crate::misc::SmolStr;

#[derive(Tsify, Debug, Serialize, Deserialize)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ConstExpr>,
    pub is_cloud: bool,
    pub is_used: bool,
}
