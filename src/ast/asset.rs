use std::path::Path;

use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub name: SmolStr,
    pub path: SmolStr,
    pub span: Span,
}

impl Asset {
    pub fn new(path: SmolStr, alias: Option<SmolStr>, span: Span) -> Self {
        let name = alias.unwrap_or_else(|| {
            Path::new(&*path)
                .file_stem()
                .unwrap()
                .to_str()
                .map(SmolStr::from)
                .unwrap_or(path.clone())
        });
        Self { name, path, span }
    }
}
