use logos::Span;

use super::Interpreter;
use crate::{
    ast::*,
    blocks::*,
};

impl Interpreter {
    pub fn run_un_op(&mut self, op: &UnOp, span: &Span, opr: &Expr) -> anyhow::Result<Value> {
        match op {
            UnOp::Not => todo!(),
            UnOp::Length => todo!(),
            UnOp::Round => todo!(),
            UnOp::Abs => todo!(),
            UnOp::Floor => todo!(),
            UnOp::Ceil => todo!(),
            UnOp::Sqrt => todo!(),
            UnOp::Sin => todo!(),
            UnOp::Cos => todo!(),
            UnOp::Tan => todo!(),
            UnOp::Asin => todo!(),
            UnOp::Acos => todo!(),
            UnOp::Atan => todo!(),
            UnOp::Ln => todo!(),
            UnOp::Log => todo!(),
            UnOp::AntiLn => todo!(),
            UnOp::AntiLog => todo!(),
            UnOp::Minus => todo!(),
        }
    }
}
