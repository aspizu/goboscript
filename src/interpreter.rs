mod block;
mod expr;
mod foreign;
mod repr;
mod stmt;

use std::{
    cell::RefCell,
    fs::File,
    path::Path,
    rc::Rc,
};

use fxhash::FxHashMap;
use logos::Span;

use crate::{
    ast::{
        EventKind,
        ListDefault,
        Name,
        Project,
        Sprite,
        Stmt,
        Value,
    },
    codegen::cmd::cmd_to_list,
    misc::SmolStr,
    vfs::RealFS,
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
        use $crate::interpreter::Exception;
        return Err(Exception::new($msg, Some($span)))
    };
    ($msg:expr) => {
        use $crate::interpreter::Exception;
        return Err(Exception::new($msg, None))
    };
}

pub struct Interpreter {
    pub vars: FxHashMap<SmolStr, Value>,
    pub lists: FxHashMap<SmolStr, Vec<Value>>,
    pub args: FxHashMap<SmolStr, Value>,
    pub answer: Value,
    pub files: Vec<File>,
}

pub fn qualify_name(name: &Name) -> SmolStr {
    match name {
        Name::Name { name, .. } => name.clone(),
        Name::DotName { lhs, rhs, .. } => format!("{lhs}.{rhs}").into(),
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: FxHashMap::default(),
            lists: FxHashMap::default(),
            args: FxHashMap::default(),
            answer: arcstr::literal!("").into(),
            files: Vec::new(),
        }
    }

    pub fn run_project(&mut self, input: &Path, project: &Project) -> ExceptionResult<()> {
        for (var_name, var) in &project.stage.vars {
            self.vars.insert(
                var_name.clone(),
                var.default
                    .clone()
                    .map(|(value, _)| value)
                    .unwrap_or(0.0.into()),
            );
        }
        for (list_name, list) in &project.stage.lists {
            let values = match &list.default {
                Some(ListDefault::Values(values)) => {
                    values.iter().map(|(value, _)| value.clone()).collect()
                }
                Some(ListDefault::Cmd(cmd)) => {
                    cmd_to_list(Rc::new(RefCell::new(RealFS::new())), cmd, input)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Value::from)
                        .collect()
                }
                None => vec![],
            };
            self.lists.insert(list_name.clone(), values);
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
