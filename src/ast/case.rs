use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::ast::{
    Expr,
    Stmt,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Case {
    pub value: Box<Expr>,
    pub body: Vec<Stmt>,
    pub span: Span,
}
