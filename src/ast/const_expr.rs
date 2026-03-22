use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    StructLiteralField,
    Value,
};
use crate::{
    ast::{
        Expr,
        Name,
    },
    misc::SmolStr,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConstExpr {
    Value {
        value: Value,
        span: Span,
    },
    EnumVariant {
        enum_name: SmolStr,
        enum_name_span: Span,
        variant_name: SmolStr,
        variant_name_span: Span,
    },
    StructLiteral {
        name: SmolStr,
        span: Span,
        fields: Vec<ConstStructLiteralField>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstStructLiteralField {
    pub name: SmolStr,
    pub name_span: Span,
    pub value: Value,
    pub value_span: Span,
}

impl ConstExpr {
    pub fn span(&self) -> Span {
        match self {
            ConstExpr::Value { span, .. } => span.clone(),
            ConstExpr::EnumVariant {
                enum_name_span,
                variant_name_span,
                ..
            } => enum_name_span.start..variant_name_span.end,
            ConstExpr::StructLiteral { span, .. } => span.clone(),
        }
    }
}

impl From<ConstExpr> for Expr {
    fn from(const_expr: ConstExpr) -> Self {
        match const_expr {
            ConstExpr::Value { value, span } => Expr::Value { value, span },
            ConstExpr::EnumVariant {
                enum_name,
                enum_name_span,
                variant_name,
                variant_name_span,
            } => Expr::Dot {
                lhs: Box::new(Expr::Name(Name::Name {
                    name: enum_name,
                    span: enum_name_span,
                })),
                rhs: variant_name,
                rhs_span: variant_name_span,
            },
            ConstExpr::StructLiteral { name, span, fields } => Expr::StructLiteral {
                name,
                span,
                fields: fields
                    .into_iter()
                    .map(|f| StructLiteralField {
                        name: f.name,
                        span: f.name_span,
                        value: Box::new(f.value.to_expr(f.value_span)),
                    })
                    .collect(),
            },
        }
    }
}
