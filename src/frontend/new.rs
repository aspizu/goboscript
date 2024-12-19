// use std::{env, fs, path::PathBuf};

// use anyhow::bail;

// use crate::config::Config;

// macro_rules! write_templates {
//     ($input:expr, $($file:expr),*) => {
//         $(
//             fs::write($input.join($file), include_str!(concat!("templates/", $file)))?;
//         )*
//     };
// }

// pub fn new(name: Option<PathBuf>, config: Config) -> anyhow::Result<()> {
//     let is_name_explicit = name.is_some();
//     let name = name.unwrap_or_else(|| env::current_dir().unwrap());
//     fs::create_dir(&name)?;
//     if name.read_dir()?.count() > 0 {
//         return Err(Error::NewDirNotEmpty { name, is_name_explicit });
//     }
//     let config_path = name.join("goboscript.toml");
//     if config != Default::default() {
//         fs::write(config_path, toml::to_string(&config).unwrap())?;
//     }
//     write_templates!(name, "stage.gs", "main.gs", "blank.svg");
//     Ok(())
// }
