use logos::Span;
use smol_str::SmolStr;

use super::enum_variant::EnumVariant;

#[derive(Debug)]
pub struct Enum {
    pub name: SmolStr,
    pub span: Span,
    pub variants: Vec<EnumVariant>,
}
