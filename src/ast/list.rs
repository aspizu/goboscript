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
    File { path: SmolStr, span: Span },
    FixedLength(ConstExpr, ConstExpr),
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

    pub fn new_array(name: SmolStr, span: Span, type_: Type, default: Vec<ConstExpr>) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::Values(default)),
            is_used: false,
        }
    }
    pub fn new_file(
        name: SmolStr,
        span: Span,
        type_: Type,
        path: SmolStr,
        path_span: Span,
    ) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::File {
                path,
                span: path_span,
            }),
            is_used: false,
        }
    }

    pub fn new_fixed_length(
        name: SmolStr,
        span: Span,
        type_: Type,
        default: ConstExpr,
        length: ConstExpr,
    ) -> Self {
        Self {
            name,
            span,
            type_,
            default: Some(ListDefault::FixedLength(default, length)),
            is_used: false,
        }
    }
}
