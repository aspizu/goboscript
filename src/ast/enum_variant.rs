use logos::Span;
use smol_str::SmolStr;

use super::value::Value;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: SmolStr,
    pub span: Span,
    pub value: Option<(Value, Span)>,
    pub is_used: bool,
}

impl EnumVariant {
    pub fn new(name: SmolStr, span: Span, value: Option<(Value, Span)>) -> Self {
        Self {
            name,
            span,
            value,
            is_used: false,
        }
    }
}
