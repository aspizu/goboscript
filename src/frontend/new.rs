use std::{
    env,
    fs,
    path::PathBuf,
};

use crate::config::Config;

pub enum NewError {
    AnyhowError(anyhow::Error),
    NewDirNotEmpty {
        name: PathBuf,
        is_name_explicit: bool,
    },
}

impl<T> From<T> for NewError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

macro_rules! write_templates {
    ($input:expr, $($file:expr),*) => {
        $(
            fs::write($input.join($file), include_str!(concat!("templates/", $file)))?;
        )*
    };
}

pub fn new(name: Option<PathBuf>, config: Config) -> Result<(), NewError> {
    let is_name_explicit = name.is_some();
    let name = name.unwrap_or_else(|| env::current_dir().unwrap());
    let _ = fs::create_dir(&name);
    if name.read_dir()?.count() > 0 {
        return Err(NewError::NewDirNotEmpty {
            name,
            is_name_explicit,
        });
    }
    let config_path = name.join("goboscript.toml");
    if config != Default::default() {
        fs::write(config_path, toml::to_string(&config).unwrap())?;
    }
    write_templates!(name, "stage.gs", "main.gs", "blank.svg");
    Ok(())
}
