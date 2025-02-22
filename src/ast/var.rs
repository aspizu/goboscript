use logos::Span;

use super::{
    type_::Type,
    ConstExpr,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ConstExpr>,
    pub is_cloud: bool,
    pub is_used: bool,
}
