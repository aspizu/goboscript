use logos::Span;

use super::{
    value,
    Interpreter,
};
use crate::{
    ast::*,
    blocks::*,
};

impl Interpreter {
    pub fn run_bin_op(
        &mut self,
        op: &BinOp,
        span: &Span,
        lhs: &Expr,
        rhs: &Expr,
    ) -> anyhow::Result<Value> {
        match op {
            BinOp::Add => add(self, span, lhs, rhs),
            BinOp::Sub => sub(self, span, lhs, rhs),
            BinOp::Mul => mul(self, span, lhs, rhs),
            BinOp::Div => div(self, span, lhs, rhs),
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
}

fn add(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = value::to_float(this.run_expr(lhs)?).unwrap_or(0.0);
    let rhs = value::to_float(this.run_expr(rhs)?).unwrap_or(0.0);
    Ok(Value::Float(lhs + rhs))
}

fn sub(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = value::to_float(this.run_expr(lhs)?).unwrap_or(0.0);
    let rhs = value::to_float(this.run_expr(rhs)?).unwrap_or(0.0);
    Ok(Value::Float(lhs - rhs))
}

fn mul(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = value::to_float(this.run_expr(lhs)?).unwrap_or(0.0);
    let rhs = value::to_float(this.run_expr(rhs)?).unwrap_or(0.0);
    Ok(Value::Float(lhs * rhs))
}

fn div(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = value::to_float(this.run_expr(lhs)?).unwrap_or(0.0);
    let rhs = value::to_float(this.run_expr(rhs)?).unwrap_or(0.0);
    if rhs == 0.0 {
        if lhs > 0.0 {
            Ok(Value::Float(f64::INFINITY))
        } else {
            Ok(Value::Float(f64::NEG_INFINITY))
        }
    } else {
        Ok(Value::Float(lhs / rhs))
    }
}

fn modulo(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = value::to_float(this.run_expr(lhs)?).unwrap_or(0.0);
    let rhs = value::to_float(this.run_expr(rhs)?).unwrap_or(0.0);
    if rhs == 0.0 {
        Ok(Value::Float(f64::NAN))
    } else {
        if lhs > 0.0 {
            Ok(Value::Float(lhs % rhs))
        } else {
            Ok(Value::Float(rhs % lhs))
        }
    }
}
