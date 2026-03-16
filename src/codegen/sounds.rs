use std::{
    cell::RefCell,
    io::{
        self,
        Write,
    },
    path::Path,
    rc::Rc,
};

use md5::{
    Digest,
    Md5,
};
use serde_json::json;

use crate::{
    ast::Asset,
    codegen::sb3::{
        Sb3,
        D,
    },
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

pub const SOUND_FORMATS: &[&str] = &["wav", "wave", "mp3"];

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn sound_entry(&mut self, name: &str, hash: &str, extension: &str) -> io::Result<()> {
        write!(self, "{{")?;
        write!(self, r#""name":{}"#, json!(name))?;
        write!(self, r#","assetId":"{hash}""#)?;
        write!(self, r#","dataFormat":"{extension}""#)?;
        write!(self, r#","md5ext":"{hash}.{extension}""#)?;
        write!(self, "}}") // sound
    }

    pub fn sound(
        &mut self,
        fs: Rc<RefCell<dyn VFS>>,
        input: &Path,
        sound: &Asset,
        d: D,
    ) -> io::Result<()> {
        let path = input.join(&*sound.path);
        let hash = self
            .costumes
            .get(&sound.path)
            .cloned()
            .map(Ok::<_, io::Error>)
            .unwrap_or_else(|| {
                let mut fs = fs.borrow_mut();
                let mut file = match fs.read_file(&path) {
                    Ok(file) => file,
                    Err(error) => {
                        d.report_io_error(
                            error,
                            Some("sound files are always relative to the project directory"),
                            &sound.span,
                        );
                        return Ok(Default::default());
                    }
                };
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher)?;
                let hash: SmolStr = format!("{:x}", hasher.finalize()).into();
                self.costumes.insert(sound.path.clone(), hash.clone());
                Ok(hash)
            })?;
        let (_, extension) = sound.path.rsplit_once('.').unwrap_or_default();
        let extension = extension.to_lowercase();
        let extension = extension.as_str();
        if !SOUND_FORMATS.contains(&extension) {
            d.report(
                DiagnosticKind::InvalidSoundFormat {
                    extension: extension.into(),
                },
                &sound.span,
            );
        }
        self.sound_entry(&sound.name, &hash, extension)
    }
}
