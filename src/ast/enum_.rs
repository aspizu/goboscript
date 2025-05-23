use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::enum_variant::EnumVariant;
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub name: SmolStr,
    pub span: Span,
    pub variants: Vec<EnumVariant>,
    pub is_used: bool,
}

impl Enum {
    pub fn new(name: SmolStr, span: Span, variants: Vec<EnumVariant>) -> Self {
        Self {
            name,
            span,
            variants,
            is_used: false,
        }
    }
}
