use logos::Span;

use super::Interpreter;
use crate::{
    ast::*,
    blocks::*,
};

impl<'a> Interpreter<'a> {
    pub fn run_bin_op(
        &mut self,
        op: &BinOp,
        span: &Span,
        lhs: &Expr,
        rhs: &Expr,
    ) -> anyhow::Result<Value> {
        match op {
            BinOp::Add => self.run_bin_op_add(span, lhs, rhs),
            BinOp::Sub => todo!(),
            BinOp::Mul => todo!(),
            BinOp::Div => todo!(),
            BinOp::Mod => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Eq => todo!(),
            BinOp::And => todo!(),
            BinOp::Or => todo!(),
            BinOp::Join => todo!(),
            BinOp::In => todo!(),
            BinOp::Of => todo!(),
            BinOp::Le => todo!(),
            BinOp::Ge => todo!(),
            BinOp::Ne => todo!(),
            BinOp::FloorDiv => todo!(),
        }
    }

    pub fn run_bin_op_add(
        &mut self,
        _span: &Span,
        lhs: &Expr,
        rhs: &Expr,
    ) -> anyhow::Result<Value> {
        let lhs = self.run_expr(lhs)?;
        let rhs = self.run_expr(rhs)?;
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Ok((lhs + rhs).into()),
            (Value::Float(lhs), Value::Float(rhs)) => Ok((lhs + rhs).into()),
            (Value::Int(lhs), Value::Float(rhs)) => Ok((lhs as f64 + rhs).into()),
            (Value::Float(lhs), Value::Int(rhs)) => Ok((lhs + rhs as f64).into()),
            _ => todo!(),
        }
    }
}
