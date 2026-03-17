use std::path::Path;

use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Costume {
    pub name: SmolStr,
    pub path: SmolStr,
    pub span: Span,
    pub rotation_center: Option<(f64, f64)>,
}

impl Costume {
    pub fn new(path: SmolStr, alias: Option<SmolStr>, rotation_center: Option<(f64, f64)>, span: Span) -> Self {
        let name = alias.unwrap_or_else(|| {
            Path::new(&*path)
                .file_stem()
                .unwrap()
                .to_str()
                .map(SmolStr::from)
                .unwrap_or(path.clone())
        });
        Self { name, path, span, rotation_center }
    }
}
