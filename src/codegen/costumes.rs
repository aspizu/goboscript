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
        let hash = &object.hash;
        let extension = &object.extension;
        if !(BITMAP_FORMATS.iter().any(|format| extension == format)
            || VECTOR_FORMATS.iter().any(|format| extension == format))
            && d.find_diagnostic_for_span(&costume.span).is_none()
        {
            d.report(
                DiagnosticKind::InvalidCostumeFormat {
                    extension: extension.clone(),
                },
                &costume.span,
            );
        }
        write!(self.json, "{{")?;
        write!(self.json, r#""name":{}"#, json!(&*costume.name))?;
        write!(self.json, r#","assetId":"{}""#, hash)?;
        if BITMAP_FORMATS.iter().any(|format| extension == format) {
            write!(
                self.json,
                r#","bitmapResolution":{}"#,
                json!(config.bitmap_resolution.unwrap_or(1))
            )?;
        }
        write!(self.json, r#","dataFormat":"{}""#, extension)?;
        write!(self.json, r#","md5ext":"{}.{}""#, hash, extension)?;
        write!(self.json, "}}") // costume
    }
}
