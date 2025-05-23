use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
};

use anyhow::Context;
use directories::ProjectDirs;

use crate::{
    ast::Project,
    diagnostic::{
        Artifact,
        SpriteDiagnostics,
    },
    interpreter::{
        Exception,
        Interpreter,
    },
    parser,
    standard_library::StandardLibrary,
    vfs::RealFS,
    visitor,
};

pub enum RunError {
    AnyhowError(anyhow::Error),
    ProjectDiagnostics(Artifact),
    Exception(Exception),
}

impl<T> From<T> for RunError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

impl From<Exception> for RunError {
    fn from(value: Exception) -> Self {
        Self::Exception(value)
    }
}

pub fn run(input: PathBuf) -> Result<(), RunError> {
    let parent = input
        .parent()
        .context("Failed to get parent directory")?
        .to_owned();
    let fs = Rc::new(RefCell::new(RealFS::new()));
    let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
    let stdlib = StandardLibrary::from_latest(&dirs.config_dir().join("std"))?;
    stdlib.fetch()?;
    let mut diagnostics = SpriteDiagnostics::new(fs.clone(), input, &stdlib);
    diagnostics.sprite_name = "stage".to_owned();
    let sprite = parser::parse(&diagnostics.translation_unit)
        .map_err(|err| {
            diagnostics.diagnostics.push(err);
        })
        .unwrap_or_default();
    let mut project = Project {
        stage: sprite,
        sprites: Default::default(),
    };
    visitor::pass0::visit_project(&mut project);
    visitor::pass1::visit_project(&mut project);
    visitor::pass2::visit_project(&mut project, &mut diagnostics, &mut Default::default());
    visitor::pass3::visit_project(&mut project);
    let mut interpreter = Interpreter::new();
    interpreter.run_project(&parent, &project)?;
    if !(diagnostics.diagnostics.is_empty()) {
        return Err(RunError::ProjectDiagnostics(Artifact {
            project,
            stage_diagnostics: diagnostics,
            sprites_diagnostics: Default::default(),
        }));
    }
    Ok(())
}
