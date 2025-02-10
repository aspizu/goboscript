use logos::Span;

use super::Value;
use crate::blocks::{
    BinOp,
    UnOp,
};

#[derive(Debug, Clone)]
pub enum ConstExpr {
    Value {
        value: Value,
        span: Span,
    },
    UnOp {
        op: UnOp,
        span: Span,
        opr: Box<ConstExpr>,
    },
    BinOp {
        op: BinOp,
        span: Span,
        lhs: Box<ConstExpr>,
        rhs: Box<ConstExpr>,
    },
}

impl ConstExpr {
    pub fn span(&self) -> Span {
        match self {
            Self::Value { span, .. } => span.clone(),
            Self::UnOp { span, .. } => span.clone(),
            Self::BinOp { span, .. } => span.clone(),
        }
    }

    pub fn evaluate(&self) -> Value {
        match self {
            Self::Value { value, .. } => value.clone(),
            Self::UnOp { op, opr, .. } => opr.evaluate().unop(*op).unwrap(),
            Self::BinOp { op, lhs, rhs, .. } => lhs.evaluate().binop(*op, &rhs.evaluate()).unwrap(),
        }
    }
}

impl UnOp {
    pub fn to_const_expr(self, span: Span, expr: ConstExpr) -> ConstExpr {
        ConstExpr::UnOp {
            op: self,
            span,
            opr: Box::new(expr),
        }
    }
}

impl BinOp {
    pub fn to_const_expr(self, span: Span, lhs: ConstExpr, rhs: ConstExpr) -> ConstExpr {
        ConstExpr::BinOp {
            op: self,
            span,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
