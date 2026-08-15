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
        let hash = &object.hash;
        let extension = &object.extension;
        if !extension.is_empty()
            && !SOUND_FORMATS.iter().any(|format| extension == format)
            && d.find_diagnostic_for_span(&sound.span).is_none()
        {
            d.report(
                DiagnosticKind::InvalidSoundFormat {
                    extension: extension.clone(),
                },
                &sound.span,
            );
        }
        write!(self.json, "{{")?;
        write!(self.json, r#""name":{}"#, json!(&*sound.name))?;
        write!(self.json, r#","assetId":"{}""#, hash)?;
        write!(self.json, r#","dataFormat":"{}""#, extension)?;
        write!(self.json, r#","md5ext":"{}.{}""#, hash, extension)?;
        write!(self.json, "}}") // sound
    }
}
