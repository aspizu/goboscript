use std::io::{
    self,
    Write,
};

use serde_json::json;

use crate::{
    ast::Asset,
    codegen::sb3::{
        Sb3,
        D,
    },
    config::Config,
    diagnostic::DiagnosticKind,
};

pub const BITMAP_FORMATS: &[&str] = &["png", "bmp", "jpeg", "jpg", "gif"];
pub const VECTOR_FORMATS: &[&str] = &["svg"];

impl Sb3 {
    pub fn costume(&mut self, config: &Config, costume: &Asset, d: D) -> io::Result<()> {
        let object = self.asset_object_store.load(costume, d);
        let hash = object.hash.clone();
        let extension = object.extension.clone();
        if !(BITMAP_FORMATS.contains(&extension.as_str())
            || VECTOR_FORMATS.contains(&extension.as_str()))
            && d.find_diagnostic_for_span(&costume.span).is_none()
        {
            d.report(
                DiagnosticKind::InvalidCostumeFormat {
                    extension: extension.as_str().into(),
                },
                &costume.span,
            );
        }
        write!(self, "{{")?;
        write!(self, r#""name":{}"#, json!(&*costume.name))?;
        write!(self, r#","assetId":"{}""#, hash)?;
        if BITMAP_FORMATS.contains(&extension.as_str()) {
            write!(
                self,
                r#","bitmapResolution":{}"#,
                json!(config.bitmap_resolution.unwrap_or(1))
            )?;
        }
        write!(self, r#","dataFormat":"{}""#, extension)?;
        write!(self, r#","md5ext":"{}.{}""#, hash, extension)?;
        write!(self, "}}") // costume
    }
}
