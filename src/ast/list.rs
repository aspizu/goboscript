use logos::Span;

use super::{
    type_::Type,
    Value,
};
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct List {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ListDefault>,
    pub is_used: bool,
}

#[derive(Debug)]
pub enum ListDefault {
    Literal(Vec<(Value, Span)>),
    Cmd(Cmd),
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
    pub fn new(name: SmolStr, span: Span, type_: Type) -> Self {
        Self {
            name,
            span,
            type_,
            default: None,
            is_used: false,
        }
    }

    pub fn new_cmd(name: SmolStr, span: Span, type_: Type, default: Cmd) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::Cmd(default)),
            is_used: false,
        }
    }

    pub fn new_array(name: SmolStr, span: Span, type_: Type, default: Vec<(Value, Span)>) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::Literal(default)),
            is_used: false,
        }
    }

    pub fn cmd(&self) -> Option<&Cmd> {
        match &self.default {
            Some(ListDefault::Cmd(cmd)) => Some(cmd),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<&[(Value, Span)]> {
        match &self.default {
            Some(ListDefault::Literal(array)) => Some(array),
            _ => None,
        }
    }
}
