use core::fmt;
use std::fmt::Display;

use logos::Span;
use serde::Serialize;

use crate::misc::SmolStr;

#[derive(Debug, Clone, Serialize)]
pub enum Type {
    Value,
    Struct { name: SmolStr, span: Span },
}

impl Type {
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value)
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct { .. })
    }

    pub fn struct_(&self) -> Option<(&SmolStr, &Span)> {
        match self {
            Self::Struct { name, span } => Some((name, span)),
            _ => None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Value => write!(f, "value"),
            Type::Struct { name, span: _ } => write!(f, "{}", name),
        }
    }
}
