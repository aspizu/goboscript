use logos::Span;
use smol_str::SmolStr;

use super::type_::Type;

#[derive(Debug)]
pub struct Var {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
}
