use std::cell::RefCell;

use logos::Span;
use smol_str::SmolStr;

use super::{value::Value, Name, StructLiteralField};
use crate::{
    blocks::{BinOp, Repr, UnOp},
    misc::Rrc,
};

#[derive(Debug)]
pub enum Expr {
    Value {
        value: Value,
        span: Span,
    },
    Name(Name),
    Dot {
        lhs: Rrc<Expr>,
        rhs: SmolStr,
        rhs_span: Span,
    },
    Arg(Name),
    Repr {
        repr: Repr,
        span: Span,
        args: Vec<Rrc<Expr>>,
    },
    UnOp {
        op: UnOp,
        span: Span,
        opr: Rrc<Expr>,
    },
    BinOp {
        op: BinOp,
        span: Span,
        lhs: Rrc<Expr>,
        rhs: Rrc<Expr>,
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
            Self::Dot { lhs, rhs_span, .. } => lhs.borrow().span().start..rhs_span.end,
            Self::Arg(name) => name.span(),
            Self::Repr { span, .. } => span.clone(),
            Self::UnOp { span, .. } => span.clone(),
            Self::BinOp { span, .. } => span.clone(),
            Self::StructLiteral { span, .. } => span.clone(),
        }
    }
}

impl UnOp {
    pub fn to_expr(self, span: Span, expr: Rrc<Expr>) -> Expr {
        Expr::UnOp {
            op: self,
            span,
            opr: expr,
        }
    }
}

impl BinOp {
    pub fn to_expr(self, span: Span, lhs: Rrc<Expr>, rhs: Rrc<Expr>) -> Expr {
        Expr::BinOp {
            op: self,
            span,
            lhs,
            rhs,
        }
    }
}

impl From<Expr> for Rrc<Expr> {
    fn from(value: Expr) -> Self {
        RefCell::new(value).into()
    }
}
