use std::path::PathBuf;

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

pub fn parse(input: PathBuf, output: Option<PathBuf>) -> Result<(), ParseError> {
    let output = output.unwrap_or_else(|| input.with_extension("json"));
    todo!()
}
