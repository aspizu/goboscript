use super::{
    foreign::foreign_func,
    qualify_name,
    value::Value,
    ExceptionResult,
    Interpreter,
};
use crate::ast::Expr;

impl Interpreter {
    pub fn run_expr(&mut self, expr: &Expr) -> ExceptionResult<Value> {
        match expr {
            Expr::Value { value, .. } => Ok(value.clone().into()),
            Expr::Name(name) => Ok(self.vars.get(&qualify_name(name)).unwrap().clone()),
            Expr::Dot { .. } => unreachable!(),
            Expr::Arg(name) => Ok(self.args.get(&qualify_name(name)).unwrap().clone()),
            Expr::Repr { repr, span, args } => self.run_repr(repr, span, args),
            Expr::FuncCall { name, span, args } => foreign_func(self, name, span, args),
            Expr::UnOp { op, span, opr } => self.run_un_op(op, span, opr),
            Expr::BinOp { op, span, lhs, rhs } => self.run_bin_op(op, span, lhs, rhs),
            Expr::StructLiteral { .. } => unreachable!(),
        }
    }
}
