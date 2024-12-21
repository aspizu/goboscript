use logos::Span;
use smol_str::SmolStr;

use super::type_::Type;

#[derive(Debug)]
pub struct List {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub cmd: Option<Cmd>,
}

#[derive(Debug)]
pub struct Cmd {
    pub program: Option<Program>,
    pub cmd: SmolStr,
    pub span: Span,
}

#[derive(Debug)]
pub struct Program {
    pub name: SmolStr,
    pub span: Span,
}
