use std::path::PathBuf;

#[derive(Debug)]
pub enum RunError {
    AnyhowError(anyhow::Error),
}

impl<T> From<T> for RunError
where T: Into<anyhow::Error>
{
    fn from(value: T) -> Self {
        Self::AnyhowError(value.into())
    }
}

pub fn run(input: PathBuf) -> Result<(), RunError> {
    Ok(())
}
