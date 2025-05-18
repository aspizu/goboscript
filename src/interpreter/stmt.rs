use logos::Span;

use super::{
    is_truthy,
    qualify_name,
    Interpreter,
};
use crate::{
    ast::*,
    blocks::*,
    misc::SmolStr,
};

impl<'a> Interpreter<'a> {
    pub fn run_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<()> {
        match stmt {
            Stmt::Repeat { times, body } => {
                let times = self.run_expr(times)?;
                let Value::Int(times) = times else { panic!() };
                for _ in 0..times {
                    self.run_stmts(body)?;
                }
                Ok(())
            }
            Stmt::Forever { body, .. } => loop {
                self.run_stmts(body)?;
            },
            Stmt::Branch {
                cond,
                if_body,
                else_body,
            } => {
                let cond = self.run_expr(cond)?;
                if is_truthy(cond) {
                    self.run_stmts(if_body)?;
                } else {
                    self.run_stmts(else_body)?;
                }
                Ok(())
            }
            Stmt::Until { cond, body } => loop {
                if is_truthy(self.run_expr(cond)?) {
                    break Ok(());
                }
                self.run_stmts(body)?;
            },
            Stmt::SetVar { name, value, .. } => {
                let name = qualify_name(name);
                let value = self.run_expr(value)?;
                self.vars.insert(name, value);
                Ok(())
            }
            Stmt::ChangeVar { name, value } => todo!(),
            Stmt::Show(name) => todo!(),
            Stmt::Hide(name) => todo!(),
            Stmt::AddToList { name, value } => todo!(),
            Stmt::DeleteList(name) => todo!(),
            Stmt::DeleteListIndex { name, index } => todo!(),
            Stmt::InsertAtList { name, index, value } => todo!(),
            Stmt::SetListIndex { name, index, value } => todo!(),
            Stmt::Block { block, span, args } => self.run_block(block, span, args),
            Stmt::ProcCall { name, span, args } => todo!(),
            Stmt::FuncCall { name, span, args } => todo!(),
            Stmt::Return { value, visited } => todo!(),
        }
    }

    pub fn run_block(
        &mut self,
        block: &Block,
        _span: &Span,
        args: &[(Option<(SmolStr, Span)>, Expr)],
    ) -> anyhow::Result<()> {
        let mut arg_values = vec![];
        for (_arg_name, arg_expr) in args {
            let arg_value = self.run_expr(arg_expr)?;
            arg_values.push(arg_value);
        }
        match block {
            Block::Say1 => {
                println!("{}", arg_values[0]);
            }
            _ => todo!(),
        }
        Ok(())
    }
}
