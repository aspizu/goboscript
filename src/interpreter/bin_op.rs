use logos::Span;

use super::{
    value::Value,
    ExceptionResult,
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
        _span: &Span,
        lhs: &Expr,
        rhs: &Expr,
    ) -> ExceptionResult<Value> {
        match op {
            BinOp::Add => add(self, lhs, rhs),
            BinOp::Sub => sub(self, lhs, rhs),
            BinOp::Mul => mul(self, lhs, rhs),
            BinOp::Div => div(self, lhs, rhs),
            BinOp::Mod => modulo(self, lhs, rhs),
            BinOp::Lt => lt(self, lhs, rhs),
            BinOp::Gt => gt(self, lhs, rhs),
            BinOp::Eq => eq(self, lhs, rhs),
            BinOp::And => and(self, lhs, rhs),
            BinOp::Or => or(self, lhs, rhs),
            BinOp::Join => join(self, lhs, rhs),
            BinOp::In => contains(self, lhs, rhs),
            BinOp::Of => letter_of(self, lhs, rhs),
            BinOp::Le => unreachable!(),
            BinOp::Ge => unreachable!(),
            BinOp::Ne => unreachable!(),
            BinOp::FloorDiv => unreachable!(),
        }
    }
}

fn add(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok((lhs + rhs).into())
}

fn sub(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok((lhs - rhs).into())
}

fn mul(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok((lhs * rhs).into())
}

fn div(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    if rhs == 0.0 {
        if lhs > 0.0 {
            Ok(f64::INFINITY.into())
        } else {
            Ok(f64::NEG_INFINITY.into())
        }
    } else {
        Ok((lhs / rhs).into())
    }
}

fn modulo(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    let mut result = lhs % rhs;
    if result / rhs < 0.0 {
        result += rhs;
    }
    Ok(result.into())
}

fn lt(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok((lhs < rhs).into())
}

fn gt(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_number();
    let rhs = this.run_expr(rhs)?.to_number();
    Ok((lhs > rhs).into())
}

fn eq(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?;
    let rhs = this.run_expr(rhs)?;
    Ok((lhs.compare(rhs) == 0.0).into())
}

fn and(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_boolean();
    let rhs = this.run_expr(rhs)?.to_boolean();
    Ok((lhs && rhs).into())
}

fn or(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_boolean();
    let rhs = this.run_expr(rhs)?.to_boolean();
    Ok((lhs || rhs).into())
}

fn join(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_string();
    let rhs = this.run_expr(rhs)?.to_string();
    Ok(format!("{lhs}{rhs}").into())
}

fn contains(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_string().to_lowercase();
    let rhs = this.run_expr(rhs)?.to_string().to_lowercase();
    Ok((lhs.contains(&rhs)).into())
}

fn letter_of(this: &mut Interpreter, lhs: &Expr, rhs: &Expr) -> ExceptionResult<Value> {
    let lhs = this.run_expr(lhs)?.to_string();
    let rhs = this.run_expr(rhs)?.to_number() - 1.0;
    if rhs < 0.0 || rhs as usize >= lhs.len() {
        return Ok("".into());
    }
    Ok(lhs.chars().nth(rhs as usize).unwrap().to_string().into())
}
