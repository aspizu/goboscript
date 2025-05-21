use super::{
    foreign::foreign_func,
    qualify_name,
    ExceptionResult,
    Interpreter,
};
use crate::ast::{
    Expr,
    Value,
};

impl Interpreter {
    pub fn run_expr(&mut self, expr: &Expr) -> ExceptionResult<Value> {
        match expr {
            Expr::Value { value, .. } => Ok(value.clone()),
            Expr::Name(name) => Ok(self.vars.get(&qualify_name(name)).unwrap().clone()),
            Expr::Dot { .. } => unreachable!(),
            Expr::Arg(name) => Ok(self.args.get(&qualify_name(name)).unwrap().clone()),
            Expr::Repr { repr, span, args } => self.run_repr(repr, span, args),
            Expr::FuncCall {
                name, span, args, ..
            } => foreign_func(self, name, span, args),
            Expr::UnOp { op, opr, .. } => Ok(Value::un_op(*op, &self.run_expr(opr)?)),
            Expr::BinOp { op, lhs, rhs, .. } => Ok(Value::bin_op(
                *op,
                &self.run_expr(lhs)?,
                &self.run_expr(rhs)?,
            )),
            Expr::StructLiteral { .. } => unreachable!(),
        }
    }
}
