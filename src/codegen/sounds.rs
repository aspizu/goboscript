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
    diagnostic::DiagnosticKind,
};

pub const SOUND_FORMATS: &[&str] = &["wav", "wave", "mp3"];

impl Sb3 {
    pub fn sound(&mut self, sound: &Asset, d: D) -> io::Result<()> {
        let object = self.asset_object_store.load(sound, d);
        let hash = object.hash.clone();
        let extension = object.extension.clone();
        if !extension.is_empty()
            && !SOUND_FORMATS.contains(&extension.as_str())
            && d.find_diagnostic_for_span(&sound.span).is_none()
        {
            d.report(
                DiagnosticKind::InvalidSoundFormat {
                    extension: extension.as_str().into(),
                },
                &sound.span,
            );
        }
        write!(self, "{{")?;
        write!(self, r#""name":{}"#, json!(&*sound.name))?;
        write!(self, r#","assetId":"{}""#, hash)?;
        write!(self, r#","dataFormat":"{}""#, extension)?;
        write!(self, r#","md5ext":"{}.{}""#, hash, extension)?;
        write!(self, "}}") // sound
    }
}
