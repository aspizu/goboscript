use std::{
    env,
    fs::{
        self,
        File,
    },
    path::PathBuf,
};

use anyhow::Context;
use directories::ProjectDirs;

use crate::{
    config::Config,
    docgen::compile::compile,
    standard_library::StandardLibrary,
};

pub enum DocgenError {
    AnyhowError(anyhow::Error),
}

impl<T> From<T> for DocgenError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

pub fn docgen(input: Option<PathBuf>, output: Option<PathBuf>) -> Result<(), DocgenError> {
    let dirs = ProjectDirs::from("com", "aspizu", "goboscript").unwrap();
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    let output = output.unwrap_or_else(|| input.join("docs.json"));
    let config_path = input.join("goboscript.toml");
    let config_src = fs::read_to_string(&config_path).unwrap_or_default();
    let config: Config = toml::from_str(&config_src)
        .with_context(|| format!("failed to parse {}", config_path.display()))?;
    let stdlib = if let Some(std) = &config.std {
        let std = std
            .strip_prefix('v')
            .unwrap_or(std)
            .parse()
            .with_context(|| format!("std version `{}` is not a valid semver version", std))?;
        StandardLibrary::new(std, &dirs.config_dir().join("std"))
    } else {
        StandardLibrary::from_latest(&dirs.config_dir().join("std"))?
    };
    stdlib.fetch()?;
    let documentation = compile(input, &stdlib)?;
    let mut file = File::create(output)?;
    serde_json::to_writer_pretty(&mut file, &documentation)?;
    Ok(())
}
