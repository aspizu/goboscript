use std::{
    f64::consts::PI,
    ops::Not,
};

use logos::Span;

use super::{
    value::Value,
    ExceptionResult,
    Interpreter,
};
use crate::{
    ast::Expr,
    blocks::UnOp,
};

impl Interpreter {
    pub fn run_un_op(&mut self, op: &UnOp, _span: &Span, opr: &Expr) -> ExceptionResult<Value> {
        match op {
            UnOp::Not => Ok(self.run_expr(opr)?.to_boolean().not().into()),
            UnOp::Length => Ok((self.run_expr(opr)?.to_string().len() as f64).into()),
            UnOp::Round => Ok(self.run_expr(opr)?.to_number().round().into()),
            UnOp::Abs => Ok(self.run_expr(opr)?.to_number().abs().into()),
            UnOp::Floor => Ok(self.run_expr(opr)?.to_number().floor().into()),
            UnOp::Ceil => Ok(self.run_expr(opr)?.to_number().ceil().into()),
            UnOp::Sqrt => Ok(self.run_expr(opr)?.to_number().sqrt().into()),
            UnOp::Sin => Ok(((self.run_expr(opr)?.to_number() * PI) / 180.0)
                .sin()
                .into()),
            UnOp::Cos => Ok(((self.run_expr(opr)?.to_number() * PI) / 180.0)
                .cos()
                .into()),
            UnOp::Tan => Ok(((self.run_expr(opr)?.to_number() * PI) / 180.0)
                .tan()
                .into()),
            UnOp::Asin => Ok(((self.run_expr(opr)?.to_number().asin() * 180.0) / PI).into()),
            UnOp::Acos => Ok(((self.run_expr(opr)?.to_number().acos() * 180.0) / PI).into()),
            UnOp::Atan => Ok(((self.run_expr(opr)?.to_number().atan() * 180.0) / PI).into()),
            UnOp::Ln => Ok(self.run_expr(opr)?.to_number().ln().into()),
            UnOp::Log => Ok(self.run_expr(opr)?.to_number().log(10.0).into()),
            UnOp::AntiLn => Ok(self.run_expr(opr)?.to_number().exp().into()),
            UnOp::AntiLog => Ok((10.0f64.powf(self.run_expr(opr)?.to_number())).into()),
            UnOp::Minus => unreachable!(),
        }
    }
}
