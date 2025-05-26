use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    type_::Type,
    ConstExpr,
};
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ListDefault>,
    pub is_used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListDefault {
    Values(Vec<ConstExpr>),
    Cmd(Cmd),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmd {
    pub program: Option<Program>,
    pub cmd: SmolStr,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn new_array(name: SmolStr, span: Span, type_: Type, default: Vec<ConstExpr>) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::Values(default)),
            is_used: false,
        }
    }

    pub fn cmd(&self) -> Option<&Cmd> {
        match &self.default {
            Some(ListDefault::Cmd(cmd)) => Some(cmd),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<&[ConstExpr]> {
        match &self.default {
            Some(ListDefault::Values(array)) => Some(array),
            _ => None,
        }
    }
}
