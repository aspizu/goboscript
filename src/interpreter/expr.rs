use super::Interpreter;
use crate::ast::*;

impl<'a> Interpreter<'a> {
    pub fn run_expr(&mut self, expr: &Expr) -> anyhow::Result<Value> {
        match expr {
            Expr::Value { value, .. } => Ok(value.clone()),
            Expr::Name(name) => match name {
                Name::Name { name, .. } => Ok(self.vars.get(name).unwrap().clone()),
                Name::DotName { lhs, rhs, .. } => Ok(self
                    .vars
                    .get(format!("{lhs}.{rhs}").as_str())
                    .unwrap()
                    .clone()),
            },
            Expr::Dot { lhs, rhs, rhs_span } => todo!(),
            Expr::Arg(name) => todo!(),
            Expr::Repr { repr, span, args } => todo!(),
            Expr::FuncCall { name, span, args } => todo!(),
            Expr::UnOp { op, span, opr } => self.run_un_op(op, span, opr),
            Expr::BinOp { op, span, lhs, rhs } => self.run_bin_op(op, span, lhs, rhs),
            Expr::StructLiteral { name, span, fields } => todo!(),
        }
    }
}
