use logos::Span;

use crate::{
    ast::*,
    blocks::*,
    diagnostic::SpriteDiagnostics,
    misc::SmolStr,
};

pub struct Interpreter<'a> {
    pub diagnostics: &'a mut SpriteDiagnostics,
}

impl<'a> Interpreter<'a> {
    pub fn new(diagnostics: &'a mut SpriteDiagnostics) -> Self {
        Self { diagnostics }
    }

    pub fn run_project(&mut self, project: &Project) -> anyhow::Result<()> {
        for event in &project.stage.events {
            if matches!(event.kind, EventKind::OnFlag) {
                self.run_stmts(&event.body)?;
            }
        }
        Ok(())
    }

    pub fn run_stmts(&mut self, stmts: &[Stmt]) -> anyhow::Result<()> {
        for stmt in stmts {
            self.run_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn run_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<()> {
        match stmt {
            Stmt::Repeat { times, body } => todo!(),
            Stmt::Forever { body, span } => todo!(),
            Stmt::Branch {
                cond,
                if_body,
                else_body,
            } => todo!(),
            Stmt::Until { cond, body } => todo!(),
            Stmt::SetVar {
                name,
                value,
                type_,
                is_local,
                is_cloud,
            } => todo!(),
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
        span: &Span,
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

    pub fn run_expr(&mut self, expr: &Expr) -> anyhow::Result<Value> {
        match expr {
            Expr::Value { value, span } => Ok(value.clone()),
            Expr::Name(name) => todo!(),
            Expr::Dot { lhs, rhs, rhs_span } => todo!(),
            Expr::Arg(name) => todo!(),
            Expr::Repr { repr, span, args } => todo!(),
            Expr::FuncCall { name, span, args } => todo!(),
            Expr::UnOp { op, span, opr } => todo!(),
            Expr::BinOp { op, span, lhs, rhs } => todo!(),
            Expr::StructLiteral { name, span, fields } => todo!(),
        }
    }
}
