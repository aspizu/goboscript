use logos::Span;
use smol_str::SmolStr;

use super::type_::Type;

#[derive(Debug)]
pub struct List {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub cmd: Option<Cmd>,
    pub is_used: bool,
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

impl List {
    pub fn new(name: SmolStr, span: Span, type_: Type, cmd: Option<Cmd>) -> Self {
        Self {
            name,
            span,
            type_,
            cmd,
            is_used: false,
        }
    }
}
