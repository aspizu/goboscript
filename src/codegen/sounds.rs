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

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn sound(&mut self, sound: &Asset, d: D) -> io::Result<()> {
        let object = self.asset_object_store.load(sound, d);
        let extension = &*object.extension;
        if !SOUND_FORMATS.contains(&extension) {
            d.report(
                DiagnosticKind::InvalidSoundFormat {
                    extension: extension.into(),
                },
                &sound.span,
            );
        }
        write!(self.zip, "{{")?;
        write!(self.zip, r#""name":{}"#, json!(&*sound.name))?;
        write!(self.zip, r#","assetId":"{}""#, object.hash)?;
        write!(self.zip, r#","dataFormat":"{}""#, extension)?;
        write!(self.zip, r#","md5ext":"{}.{}""#, object.hash, extension)?;
        write!(self.zip, "}}") // sound
    }
}
