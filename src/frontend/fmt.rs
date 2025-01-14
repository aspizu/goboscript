use std::{
    env,
    path::PathBuf,
};

use glob::glob;

use crate::fmt;

pub enum FmtError {
    AnyhowError(anyhow::Error),
}

impl<T> From<T> for FmtError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

impl From<fmt::FmtError> for FmtError {
    fn from(value: fmt::FmtError) -> Self {
        todo!()
    }
}

pub fn fmt(input: Option<PathBuf>) -> Result<(), FmtError> {
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    if input.is_file() {
        fmt::format_file(input)?;
    } else {
        for path in glob(input.join("**/*.gs").to_str().unwrap())? {
            let path = path?;
            fmt::format_file(path)?;
        }
    }
    Ok(())
}
