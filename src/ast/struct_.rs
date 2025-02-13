use logos::Span;
use serde::Serialize;

use super::struct_field::StructField;
use crate::misc::SmolStr;

#[derive(Debug, Serialize)]
pub struct Struct {
    pub name: SmolStr,
    pub span: Span,
    pub fields: Vec<StructField>,
    pub is_used: bool,
}

impl Struct {
    pub fn new(name: SmolStr, span: Span, fields: Vec<(SmolStr, Span)>) -> Self {
        Self {
            name,
            span,
            fields: fields
                .into_iter()
                .map(|(name, span)| StructField {
                    name,
                    span,
                    is_used: false,
                })
                .collect(),
            is_used: false,
        }
    }
}
