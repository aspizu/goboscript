use logos::Span;

use super::{
    struct_field::StructField,
    Value,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Struct {
    pub name: SmolStr,
    pub span: Span,
    pub fields: Vec<StructField>,
    pub is_used: bool,
}

impl Struct {
    pub fn new(
        name: SmolStr,
        span: Span,
        fields: Vec<(SmolStr, Span, Option<(Value, Span)>)>,
    ) -> Self {
        Self {
            name,
            span,
            fields: fields
                .into_iter()
                .map(|(name, span, default)| StructField {
                    name,
                    span,
                    default,
                    is_used: false,
                })
                .collect(),
            is_used: false,
        }
    }
}
