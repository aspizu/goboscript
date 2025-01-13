use std::{
    env,
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use fxhash::FxHashMap;

use crate::{
    ast::{Project, Sprite},
    codegen::sb3::Sb3,
    config::Config,
    diagnostic::{ProjectDiagnostics, SpriteDiagnostics},
    misc::SmolStr,
    parser, visitor,
};

pub enum BuildError {
    AnyhowError(anyhow::Error),
    ProjectDiagnostics(ProjectDiagnostics),
}

impl<T> From<T> for BuildError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

impl From<ProjectDiagnostics> for BuildError {
    fn from(value: ProjectDiagnostics) -> Self {
        Self::ProjectDiagnostics(value)
    }
}

pub fn build(input: Option<PathBuf>, output: Option<PathBuf>) -> Result<(), BuildError> {
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    let canonical_input = input.canonicalize()?;
    let project_name = canonical_input.file_name().unwrap().to_str().unwrap();
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let config_path = input.join("goboscript.toml");
    let config_src = fs::read_to_string(&config_path).unwrap_or_default();
    let config: Config = toml::from_str(&config_src)
        .with_context(|| format!("failed to parse {}", config_path.display()))?;
    let stage_path = input.join("stage.gs");
    if !stage_path.is_file() {
        return Err(anyhow!("{} not found", stage_path.display()).into());
    }
    let mut stage_diagnostics = SpriteDiagnostics::new(stage_path);
    let stage = parser::parse(&stage_diagnostics.translation_unit)
        .map_err(|err| {
            stage_diagnostics.diagnostics.push(err);
        })
        .unwrap_or_default();
    let mut sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics> = Default::default();
    let mut sprites: FxHashMap<SmolStr, Sprite> = Default::default();
    for sprite_path in fs::read_dir(&input)? {
        let sprite_path = sprite_path?.path();
        if sprite_path.file_stem().is_some_and(|stem| stem == "stage") {
            continue;
        }
        if !sprite_path
            .extension()
            .is_some_and(|extension| extension == "gs")
        {
            continue;
        }
        if !sprite_path.is_file() {
            continue;
        }
        let sprite_name: SmolStr = sprite_path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .into();
        let mut sprite_diagnostics = SpriteDiagnostics::new(sprite_path);
        let sprite = parser::parse(&sprite_diagnostics.translation_unit)
            .map_err(|err| sprite_diagnostics.diagnostics.push(err))
            .unwrap_or_default();
        sprites_diagnostics.insert(sprite_name.clone(), sprite_diagnostics);
        sprites.insert(sprite_name, sprite);
    }
    let mut project = Project { stage, sprites };
    if !(stage_diagnostics.diagnostics.is_empty()
        && sprites_diagnostics
            .values()
            .all(|sprite_diagnostics| sprite_diagnostics.diagnostics.is_empty()))
    {
        return Err(ProjectDiagnostics {
            project,
            stage_diagnostics,
            sprites_diagnostics,
        }
        .into());
    }
    visitor::pass0::visit_project(&mut project);
    visitor::pass2::visit_project(&mut project);
    visitor::pass1::visit_project(
        &mut project,
        &mut stage_diagnostics,
        &mut sprites_diagnostics,
    );
    visitor::pass3::visit_project(&mut project);
    log::info!("{:#?}", project);
    let mut sb3 = Sb3::new(BufWriter::new(File::create(&output)?));
    sb3.project(
        &input,
        &project,
        &config,
        &mut stage_diagnostics,
        &mut sprites_diagnostics,
    )?;
    if !(stage_diagnostics.diagnostics.is_empty()
        && sprites_diagnostics
            .values()
            .all(|sprite_diagnostics| sprite_diagnostics.diagnostics.is_empty()))
    {
        return Err(ProjectDiagnostics {
            project,
            stage_diagnostics,
            sprites_diagnostics,
        }
        .into());
    }
    Ok(())
}
