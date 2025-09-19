use fxhash::FxHashMap;
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    value::Value,
    ConstExpr,
    Name,
    StructLiteralField,
};
use crate::{
    blocks::{
        BinOp,
        Repr,
        UnOp,
    },
    misc::SmolStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Value {
        value: Value,
        span: Span,
    },
    Name(Name),
    Dot {
        lhs: Box<Expr>,
        rhs: SmolStr,
        rhs_span: Span,
    },
    Arg(Name),
    Repr {
        repr: Repr,
        span: Span,
        args: Vec<Expr>,
    },
    FuncCall {
        name: SmolStr,
        span: Span,
        args: Vec<Expr>,
        kwargs: FxHashMap<SmolStr, (Span, Expr)>,
    },
    UnOp {
        op: UnOp,
        span: Span,
        opr: Box<Expr>,
    },
    BinOp {
        op: BinOp,
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    StructLiteral {
        name: SmolStr,
        span: Span,
        fields: Vec<StructLiteralField>,
    },
    Property {
        object: Box<Expr>,
        property: SmolStr,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Value { span, .. } => span.clone(),
            Self::Name(name) => name.span(),
            Self::Dot { lhs, rhs_span, .. } => lhs.span().start..rhs_span.end,
            Self::Arg(name) => name.span(),
            Self::Repr { span, .. } => span.clone(),
            Self::FuncCall { span, .. } => span.clone(),
            Self::UnOp { span, .. } => span.clone(),
            Self::BinOp { span, .. } => span.clone(),
            Self::StructLiteral { span, .. } => span.clone(),
            Self::Property { span, .. } => span.clone(),
        }
    }
}

impl UnOp {
    pub fn to_expr(self, span: Span, expr: Expr) -> Expr {
        Expr::UnOp {
            op: self,
            span,
            opr: Box::new(expr),
        }
    }
}

impl BinOp {
    pub fn to_expr(self, span: Span, lhs: Expr, rhs: Expr) -> Expr {
        Expr::BinOp {
            op: self,
            span,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl Value {
    pub fn to_expr(self, span: Span) -> Expr {
        Expr::Value { value: self, span }
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
            } => Expr::Name(Name::DotName {
                lhs: enum_name,
                lhs_span: enum_name_span,
                rhs: variant_name,
                rhs_span: variant_name_span,
                is_generated: false,
            }),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Simple expressions that can be compared
            (Expr::Value { value: v1, .. }, Expr::Value { value: v2, .. }) => v1 == v2,
            (Expr::Name(n1), Expr::Name(n2)) => n1 == n2,
            (Expr::Arg(n1), Expr::Arg(n2)) => n1 == n2,

            // Complex expressions always return false
            _ => false,
        }
    }
}
