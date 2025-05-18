mod bin_op;
mod block;
mod expr;
mod repr;
mod stmt;
mod un_op;
mod value;

use fxhash::FxHashMap;
use logos::Span;
use value::Value;

use crate::{
    ast::{
        EventKind,
        Name,
        Project,
        Sprite,
        Stmt,
    },
    misc::SmolStr,
};

#[derive(Debug, Clone)]
pub struct Exception {
    pub message: SmolStr,
    pub span: Option<Span>,
}

pub type ExceptionResult<T> = Result<T, Exception>;

impl Exception {
    pub fn new<T: Into<SmolStr>>(message: T, span: Option<Span>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

#[macro_export]
macro_rules! throw {
    ($msg:expr, $span:expr) => {
        use crate::interpreter::Exception;
        return Err(Exception::new($msg, Some($span)))
    };
    ($msg:expr) => {
        use crate::interpreter::Exception;
        return Err(Exception::new($msg, None))
    };
}

#[derive(Default)]
pub struct Interpreter {
    pub vars: FxHashMap<SmolStr, Value>,
    pub args: FxHashMap<SmolStr, Value>,
}

pub fn qualify_name(name: &Name) -> SmolStr {
    match name {
        Name::Name { name, .. } => name.clone(),
        Name::DotName { lhs, rhs, .. } => format!("{lhs}.{rhs}").into(),
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run_project(&mut self, project: &Project) -> ExceptionResult<()> {
        for (var_name, var) in &project.stage.vars {
            self.vars.insert(
                var_name.clone(),
                var.default
                    .as_ref()
                    .map(|v| v.evaluate())
                    .unwrap_or(0.into())
                    .into(),
            );
        }
        for event in &project.stage.events {
            if matches!(event.kind, EventKind::OnFlag) {
                self.run_script(&project.stage, &event.body)?;
            }
        }
        Ok(())
    }

    pub fn run_stmts(&mut self, sprite: &Sprite, stmts: &[Stmt]) -> ExceptionResult<()> {
        for stmt in stmts {
            self.run_stmt(sprite, stmt)?;
        }
        Ok(())
    }

    pub fn run_script(&mut self, sprite: &Sprite, script: &[Stmt]) -> ExceptionResult<()> {
        match self.run_stmts(sprite, script) {
            Ok(()) => Ok(()),
            Err(Exception { message, .. }) if message == "__stop_this_script__" => Ok(()),
            Err(e) => Err(e),
        }
    }
}
