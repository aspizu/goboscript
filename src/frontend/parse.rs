use std::{
    fs::File,
    path::PathBuf,
};

use anyhow::Context;
use directories::ProjectDirs;
use semver::Version;

use crate::{
    diagnostic::SpriteDiagnostics,
    parser,
    standard_library::StandardLibrary,
};

pub enum ParseError {
    AnyhowError(anyhow::Error),
}

impl<T> From<T> for ParseError
where T: Into<anyhow::Error>
{
    fn from(error: T) -> Self {
        Self::AnyhowError(error.into())
    }
}

pub fn parse(
    input: PathBuf,
    output: Option<PathBuf>,
    std: Option<Version>,
) -> Result<(), ParseError> {
    let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
    let cache_path = dirs.cache_dir().join("stdlib");
    let stdlib = if let Some(version) = std {
        StandardLibrary::new(version, &cache_path)
    } else {
        StandardLibrary::from_latest(&cache_path)?
    };
    let output = output.unwrap_or_else(|| input.with_extension("json"));
    let mut diagnostics = SpriteDiagnostics::new(input.clone(), &stdlib)
        .with_context(|| format!("failed to open {}", input.display()))?;
    let sprite = parser::parse(&diagnostics.translation_unit)
        .map_err(|err| {
            diagnostics.diagnostics.push(err);
        })
        .unwrap_or_default();
    let mut file = File::create(&output)
        .with_context(|| format!("Failed to create output JSON file {}", output.display()))?;
    serde_json::to_writer_pretty(&mut file, &sprite)
        .with_context(|| format!("Failed to serialize JSON to {}", output.display()))?;
    Ok(())
}
