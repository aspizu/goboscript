use logos::Span;
use smol_str::SmolStr;

use super::value::Value;

#[derive(Debug)]
pub struct EnumVariant {
    pub name: SmolStr,
    pub span: Span,
    pub value: Option<(Value, Span)>,
}
