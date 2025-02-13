use logos::Span;
use serde::Serialize;

use super::type_::Type;
use crate::misc::SmolStr;

#[derive(Debug, Serialize)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub is_cloud: bool,
    pub is_used: bool,
}
