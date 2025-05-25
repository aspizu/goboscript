use std::path::Path;

use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {
    pub name: SmolStr,
    pub path: SmolStr,
    pub span: Span,
}

impl Sound {
    pub fn new(path: SmolStr, alias: Option<SmolStr>, span: Span) -> Self {
        let name = alias.unwrap_or_else(|| {
            Path::new(&*path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .into()
        });
        Self { name, path, span }
    }
}
