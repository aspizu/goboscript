use std::path::PathBuf;

use directories::ProjectDirs;

use crate::{
    ast::Project,
    diagnostic::{
        ProjectDiagnostics,
        SpriteDiagnostics,
    },
    interpreter::Interpreter,
    parser,
    standard_library::StandardLibrary,
    visitor,
};

pub enum RunError {
    AnyhowError(anyhow::Error),
    ProjectDiagnostics(ProjectDiagnostics),
}

impl<T> From<T> for RunError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

pub fn run(input: PathBuf) -> Result<(), RunError> {
    let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
    let stdlib = StandardLibrary::from_latest(&dirs.config_dir().join("std"))?;
    stdlib.fetch()?;
    let mut diagnostics = SpriteDiagnostics::new(input, &stdlib);
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
    interpreter.run_project(&project)?;
    if !(diagnostics.diagnostics.is_empty()) {
        return Err(RunError::ProjectDiagnostics(ProjectDiagnostics {
            project,
            stage_diagnostics: diagnostics,
            sprites_diagnostics: Default::default(),
        }));
    }
    Ok(())
}
