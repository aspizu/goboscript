use std::{
    env,
    fs::{self, create_dir},
    path::PathBuf,
};

use anyhow::{bail, Result};

use crate::config::Config;

macro_rules! write_templates {
    ($input:expr, $($file:expr),*) => {
        $(
            fs::write($input.join($file), include_str!(concat!("templates/", $file)))?;
        )*
    };
}

pub fn new(input: Option<PathBuf>, config: Config) -> Result<()> {
    let input_explicit = input.is_some();
    let input = if let Some(input) = input { input } else { env::current_dir()? };
    if let Err(err) = create_dir(&input) {
        if !matches!(err.kind(), std::io::ErrorKind::AlreadyExists) {
            bail!(err);
        }
    }
    if input.read_dir()?.count() > 0 {
        if input_explicit {
            bail!("directory is not empty");
        }
        bail!("current directory is not empty, provide a `--name` argument");
    }
    if config != Default::default() {
        fs::write(input.join("goboscript.toml"), toml::to_string(&config)?)?;
    }
    write_templates!(input, "stage.gs", "main.gs", "blank.svg");
    Ok(())
}
