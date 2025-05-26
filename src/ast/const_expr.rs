use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::Value;
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConstExpr {
    Value {
        value: Value,
        span: Span,
    },
    EnumVariant {
        enum_name: SmolStr,
        enum_name_span: Span,
        variant_name: SmolStr,
        variant_name_span: Span,
    },
}

impl ConstExpr {
    pub fn span(&self) -> Span {
        match self {
            ConstExpr::Value { span, .. } => span.clone(),
            ConstExpr::EnumVariant {
                enum_name_span,
                variant_name_span,
                ..
            } => enum_name_span.start..variant_name_span.end,
        }
    }
}
