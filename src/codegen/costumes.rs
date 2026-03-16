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

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn costume(&mut self, config: &Config, costume: &Asset, d: D) -> io::Result<()> {
        let object = self.asset_object_store.load(costume, d);
        let extension = &*object.extension;
        if !(BITMAP_FORMATS.contains(&extension) || VECTOR_FORMATS.contains(&extension)) {
            d.report(
                DiagnosticKind::InvalidCostumeFormat {
                    extension: extension.into(),
                },
                &costume.span,
            );
        }
        write!(self.zip, "{{")?;
        write!(self.zip, r#""name":{}"#, json!(&*costume.name))?;
        write!(self.zip, r#","assetId":"{}""#, object.hash)?;
        if BITMAP_FORMATS.contains(&extension) {
            write!(
                self.zip,
                r#","bitmapResolution":{}"#,
                json!(config.bitmap_resolution.unwrap_or(1))
            )?;
        }
        write!(self.zip, r#","dataFormat":"{}""#, extension)?;
        write!(self.zip, r#","md5ext":"{}.{}""#, object.hash, extension)?;
        write!(self.zip, "}}") // costume
    }
}
