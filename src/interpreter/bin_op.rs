use logos::Span;

use super::{
    value::{
        self,
        Value,
    },
    Interpreter,
};
use crate::{
    ast::Expr,
    blocks::BinOp,
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
            BinOp::Mod => modulo(self, span, lhs, rhs),
            BinOp::Lt => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Eq => eq(self, span, lhs, rhs),
            BinOp::And => todo!(),
            BinOp::Or => todo!(),
            BinOp::Join => join(self, span, lhs, rhs),
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
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok(Value::Number(lhs + rhs))
}

fn sub(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok(Value::Number(lhs - rhs))
}

fn mul(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok(Value::Number(lhs * rhs))
}

fn div(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    if rhs == 0.0 {
        if lhs > 0.0 {
            Ok(Value::Number(f64::INFINITY))
        } else {
            Ok(Value::Number(f64::NEG_INFINITY))
        }
    } else {
        Ok(Value::Number(lhs / rhs))
    }
}

fn modulo(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    let mut result = lhs % rhs;
    if result / rhs < 0.0 {
        result += rhs;
    }
    Ok(Value::Number(result))
}

fn eq(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?;
    let rhs = this.run_expr(rhs)?;
    Ok(Value::Boolean(lhs.compare(rhs) == 0.0))
}

fn join(this: &mut Interpreter, _span: &Span, lhs: &Expr, rhs: &Expr) -> anyhow::Result<Value> {
    let lhs = this.run_expr(lhs)?.to_string();
    let rhs = this.run_expr(rhs)?.to_string();
    Ok(Value::String(format!("{lhs}{rhs}").into()))
}
