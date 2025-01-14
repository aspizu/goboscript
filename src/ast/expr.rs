use logos::Span;

use super::{
    value::Value,
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

#[derive(Debug, Clone)]
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
        args: Vec<(Option<(SmolStr, Span)>, Expr)>,
    },
    FuncCall {
        name: SmolStr,
        span: Span,
        args: Vec<(Option<(SmolStr, Span)>, Expr)>,
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
