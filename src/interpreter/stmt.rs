use fxhash::FxHashMap;
use logos::Span;

use super::{
    foreign::foreign_proc,
    qualify_name,
    ExceptionResult,
    Interpreter,
};
use crate::{
    ast::{
        Expr,
        Sprite,
        Stmt,
        Value,
    },
    misc::SmolStr,
    throw,
};

impl Interpreter {
    pub fn run_stmt(&mut self, sprite: &Sprite, stmt: &Stmt) -> ExceptionResult<()> {
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
            Stmt::Block {
                block, span, args, ..
            } => self.run_block(block, span, args),
            Stmt::ProcCall {
                name, span, args, ..
            } => self.run_proc_call(sprite, name, span, args),
            Stmt::FuncCall {
                name, span, args, ..
            } => self.run_func_call(sprite, name, span, args),
            Stmt::Return { value, visited } => todo!(),
        }
    }

    pub fn run_proc_call(
        &mut self,
        sprite: &Sprite,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
    ) -> ExceptionResult<()> {
        let Some(proc) = sprite.procs.get(name) else {
            if !foreign_proc(self, name, span, args)? {
                throw!(format!("Procedure {} not found", name), span.clone());
            }
            return Ok(());
        };
        if proc.args.len() != args.len() {
            throw!(
                format!("Expected {} arguments, got {}", proc.args.len(), args.len()),
                span.clone()
            );
        }
        let mut new_args = FxHashMap::default();
        for (arg, arg_expr) in proc.args.iter().zip(args) {
            let arg_value = self.run_expr(arg_expr)?;
            new_args.insert(arg.name.clone(), arg_value);
        }
        let proc_definition = sprite.proc_definitions.get(name).unwrap();
        let previous_args = std::mem::take(&mut self.args);
        self.args = new_args;
        self.run_script(sprite, proc_definition)?;
        self.args = previous_args;
        Ok(())
    }

    pub fn run_func_call(
        &mut self,
        sprite: &Sprite,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
    ) -> ExceptionResult<()> {
        let func = sprite.funcs.get(name).unwrap();
        if func.args.len() != args.len() {
            throw!(
                format!("Expected {} arguments, got {}", func.args.len(), args.len()),
                span.clone()
            );
        }
        let mut new_args = FxHashMap::default();
        for (arg, arg_expr) in func.args.iter().zip(args) {
            let arg_value = self.run_expr(arg_expr)?;
            new_args.insert(arg.name.clone(), arg_value);
        }
        let func_definition = sprite.func_definitions.get(name).unwrap();
        let previous_args = std::mem::take(&mut self.args);
        self.args = new_args;
        self.run_script(sprite, func_definition)?;
        self.args = previous_args;
        Ok(())
    }
}
