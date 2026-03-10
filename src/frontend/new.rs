use std::{
    env,
    fs::{
        self,
        File,
    },
    io::Write,
    path::PathBuf,
    process::Command,
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
            let path = PathBuf::from($file);
            fs::create_dir_all($input.join(path.parent().unwrap()))?;
            fs::write($input.join(path), include_str!(concat!("templates/", $file)))?;
        )*
    };
}

pub fn new(
    name: Option<PathBuf>,
    no_git: bool,
    makefile: bool,
    config: Config,
) -> Result<(), NewError> {
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
    let mut file = File::create(config_path)?;
    let toml_data = toml::to_string(&config).unwrap();
    file.write_all(
        "# Configuration Reference: <https://aspizu.github.io/goboscript/configuration>\n"
            .as_bytes(),
    )?;
    file.write_all(toml_data.as_bytes())?;
    write_templates!(&name, "stage.gs", "main.gs", "assets/blank.svg");
    if makefile {
        write_templates!(&name, "Makefile");
    }
    if !no_git {
        let _ = Command::new("git").arg("init").arg(&name).spawn();
        write_templates!(&name, ".gitignore");
    }
    Ok(())
}
