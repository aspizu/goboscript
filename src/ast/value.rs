mod binop;
mod unop;
use std::fmt::{
    self,
    Display,
};

use logos::Span;

use super::{
    ConstExpr,
    Expr,
};
use crate::misc::SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(SmolStr),
    Bool(bool)
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::String(string) => write!(f, "{}", string),
            Self::Bool(bool) => write!(f, "{}", bool)
        }
    }
}

impl From<i32> for Value {
    fn from(int: i32) -> Self {
        Self::Int(int as i64)
    }
}

impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Self::Int(int)
    }
}

impl From<usize> for Value {
    fn from(int: usize) -> Self {
        Self::Int(int as i64)
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl From<&SmolStr> for Value {
    fn from(string: &SmolStr) -> Self {
        Self::String(string.clone())
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Self::String(string.into())
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Self::String(string.into())
    }
}

impl From<SmolStr> for Value {
    fn from(string: SmolStr) -> Self {
        Self::String(string)
    }
}

impl From<bool> for Value{
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl Value {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_expr(self, span: Span) -> Expr {
        Expr::Value { value: self, span }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_const_expr(self, span: Span) -> ConstExpr {
        ConstExpr::Value { value: self, span }
    }

    pub fn map_unless_infinity<F>(self, value: F) -> Self
    where F: FnOnce(Self) -> Self {
        match self {
            Self::String(s) if s == "Infinity" => "Infinity".into(),
            Self::String(s) if s == "-Infinity" => "-Infinity".into(),
            _ => value(self),
        }
    }
}
