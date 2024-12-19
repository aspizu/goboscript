use logos::Span;
use smol_str::SmolStr;

#[derive(Debug)]
pub struct StructField {
    pub name: SmolStr,
    pub span: Span,
}
