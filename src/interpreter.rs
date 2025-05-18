mod bin_op;
mod expr;
mod stmt;
mod un_op;
mod value;

use fxhash::FxHashMap;
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

    pub fn run_project(&mut self, project: &Project) -> anyhow::Result<()> {
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
                self.run_stmts(&project.stage, &event.body)?;
            }
        }
        Ok(())
    }

    pub fn run_stmts(&mut self, sprite: &Sprite, stmts: &[Stmt]) -> anyhow::Result<()> {
        for stmt in stmts {
            self.run_stmt(sprite, stmt)?;
        }
        Ok(())
    }
}
