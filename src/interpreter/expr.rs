use super::{
    qualify_name,
    value::Value,
    Interpreter,
};
use crate::ast::Expr;

impl Interpreter {
    pub fn run_expr(&mut self, expr: &Expr) -> anyhow::Result<Value> {
        match expr {
            Expr::Value { value, .. } => Ok(value.clone().into()),
            Expr::Name(name) => Ok(self.vars.get(&qualify_name(name)).unwrap().clone()),
            Expr::Dot { .. } => unreachable!(),
            Expr::Arg(name) => Ok(self.args.get(&qualify_name(name)).unwrap().clone()),
            Expr::Repr { repr, span, args } => todo!(),
            Expr::FuncCall { .. } => unreachable!(),
            Expr::UnOp { op, span, opr } => self.run_un_op(op, span, opr),
            Expr::BinOp { op, span, lhs, rhs } => self.run_bin_op(op, span, lhs, rhs),
            Expr::StructLiteral { .. } => unreachable!(),
        }
    }
}
