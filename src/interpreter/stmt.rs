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
        ListIndex,
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
            Stmt::Show(_) => unimplemented!(),
            Stmt::Hide(_) => unimplemented!(),
            Stmt::AddToList { name, value } => {
                let name = qualify_name(name);
                let value = self.run_expr(value)?;
                let list = self.lists.get_mut(&name).unwrap();
                list.push(value);
                Ok(())
            }
            Stmt::DeleteList(name) => {
                let name = qualify_name(name);
                let list = self.lists.get_mut(&name).unwrap();
                list.clear();
                Ok(())
            }
            Stmt::DeleteListIndex { name, index } => {
                let name = qualify_name(name);
                // TODO: implement LIST_ALL
                let index = self.run_expr(index)?;
                let list = self.lists.get_mut(&name).unwrap();
                match index.to_list_index(list.len()) {
                    Some(ListIndex::All) => {
                        list.clear();
                    }
                    Some(ListIndex::Index(index)) => {
                        list.remove(index);
                    }
                    None => {}
                }
                Ok(())
            }
            Stmt::InsertAtList { name, index, value } => {
                let name = qualify_name(name);
                let index = self.run_expr(index)?;
                let value = self.run_expr(value)?;
                let list = self.lists.get_mut(&name).unwrap();
                match index.to_list_index(list.len()) {
                    Some(ListIndex::All) => {}
                    Some(ListIndex::Index(index)) => {
                        list.insert(index, value);
                    }
                    None => {}
                }
                Ok(())
            }
            Stmt::SetListIndex { name, index, value } => {
                let name = qualify_name(name);
                let index = self.run_expr(index)?;
                let value = self.run_expr(value)?;
                let list = self.lists.get_mut(&name).unwrap();
                match index.to_list_index(list.len()) {
                    Some(ListIndex::All) => {}
                    Some(ListIndex::Index(index)) => {
                        list[index] = value;
                    }
                    None => {}
                }
                Ok(())
            }
            Stmt::Block {
                block, span, args, ..
            } => self.run_block(block, span, args),
            Stmt::ProcCall {
                name, span, args, ..
            } => self.run_proc_call(sprite, name, span, args),
            Stmt::FuncCall {
                name, span, args, ..
            } => self.run_func_call(sprite, name, span, args),
            Stmt::Return { .. } => unreachable!(),
        }
    }

    pub fn run_proc_call(
        &mut self,
        sprite: &Sprite,
        name: &SmolStr,
        span: &Span,
        args: &[Expr],
    ) -> ExceptionResult<()> {
        let Some(_proc) = sprite.procs.get(name) else {
            if !foreign_proc(self, name, span, args)? {
                throw!(format!("Procedure {} not found", name), span.clone());
            }
            return Ok(());
        };
        let signature = &sprite.proc_args[name];
        if signature.len() != args.len() {
            throw!(
                format!("Expected {} arguments, got {}", signature.len(), args.len()),
                span.clone()
            );
        }
        let mut new_args = FxHashMap::default();
        for (arg, arg_expr) in signature.iter().zip(args) {
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
        let signature = &sprite.func_args[name];
        if signature.len() != args.len() {
            throw!(
                format!("Expected {} arguments, got {}", signature.len(), args.len()),
                span.clone()
            );
        }
        let mut new_args = FxHashMap::default();
        for (arg, arg_expr) in signature.iter().zip(args) {
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
