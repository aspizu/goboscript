use std::path::PathBuf;

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
    todo!()
}
