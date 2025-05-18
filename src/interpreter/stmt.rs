use logos::Span;

use super::{
    qualify_name,
    value::Value,
    Interpreter,
};
use crate::{
    ast::{
        Expr,
        Sprite,
        Stmt,
    },
    blocks::Block,
    misc::SmolStr,
};

impl Interpreter {
    pub fn run_stmt(&mut self, sprite: &Sprite, stmt: &Stmt) -> anyhow::Result<()> {
        match stmt {
            Stmt::Repeat { times, body } => {
                let times = self.run_expr(times)?.to_number() as usize;
                for _ in 0..times {
                    self.run_stmts(sprite, body)?;
                }
                Ok(())
            }
            Stmt::Forever { body, .. } => loop {
                self.run_stmts(sprite, body)?;
            },
            Stmt::Branch {
                cond,
                if_body,
                else_body,
            } => {
                let cond = self.run_expr(cond)?;
                if cond.to_boolean() {
                    self.run_stmts(sprite, if_body)?;
                } else {
                    self.run_stmts(sprite, else_body)?;
                }
                Ok(())
            }
            Stmt::Until { cond, body } => loop {
                if self.run_expr(cond)?.to_boolean() {
                    break Ok(());
                }
                self.run_stmts(sprite, body)?;
            },
            Stmt::SetVar { name, value, .. } => {
                let name = qualify_name(name);
                let value = self.run_expr(value)?;
                self.vars.insert(name, value);
                Ok(())
            }
            Stmt::ChangeVar { name, value } => {
                let name = qualify_name(name);
                let value = self.run_expr(value)?.to_number();
                let current_value = self.vars.get(&name).unwrap().clone().to_number();
                self.vars.insert(name, Value::Number(current_value + value));
                Ok(())
            }
            Stmt::Show(name) => todo!(),
            Stmt::Hide(name) => todo!(),
            Stmt::AddToList { name, value } => todo!(),
            Stmt::DeleteList(name) => todo!(),
            Stmt::DeleteListIndex { name, index } => todo!(),
            Stmt::InsertAtList { name, index, value } => todo!(),
            Stmt::SetListIndex { name, index, value } => todo!(),
            Stmt::Block { block, span, args } => self.run_block(block, span, args),
            Stmt::ProcCall { name, span, args } => self.run_proc_call(sprite, name, span, args),
            Stmt::FuncCall { name, span, args } => self.run_func_call(sprite, name, span, args),
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
                println!("{}", arg_values[0].clone().to_string());
            }
            _ => todo!(),
        }
        Ok(())
    }

    pub fn run_proc_call(
        &mut self,
        sprite: &Sprite,
        name: &SmolStr,
        _span: &Span,
        args: &[(Option<(SmolStr, Span)>, Expr)],
    ) -> anyhow::Result<()> {
        let previous_args = std::mem::take(&mut self.args);
        let proc = sprite.procs.get(name).unwrap();
        for (arg, (_arg_name, arg_expr)) in proc.args.iter().zip(args) {
            let arg_value = self.run_expr(arg_expr)?;
            self.args.insert(arg.name.clone(), arg_value);
        }
        let proc_definition = sprite.proc_definitions.get(name).unwrap();
        self.run_stmts(sprite, &proc_definition)?;
        self.args = previous_args;
        Ok(())
    }

    pub fn run_func_call(
        &mut self,
        sprite: &Sprite,
        name: &SmolStr,
        _span: &Span,
        args: &[(Option<(SmolStr, Span)>, Expr)],
    ) -> anyhow::Result<()> {
        let previous_args = std::mem::take(&mut self.args);
        let func = sprite.funcs.get(name).unwrap();
        for (arg, (_arg_name, arg_expr)) in func.args.iter().zip(args) {
            let arg_value = self.run_expr(arg_expr)?;
            self.args.insert(arg.name.clone(), arg_value);
        }
        let func_definition = sprite.func_definitions.get(name).unwrap();
        self.run_stmts(sprite, &func_definition)?;
        self.args = previous_args;
        Ok(())
    }
}
