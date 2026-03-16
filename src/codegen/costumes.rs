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
    config::Config,
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

pub const BITMAP_FORMATS: &[&str] = &["png", "bmp", "jpeg", "jpg", "gif"];
pub const VECTOR_FORMATS: &[&str] = &["svg"];

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn costume_entry(
        &mut self,
        config: &Config,
        name: &str,
        hash: &str,
        extension: &str,
    ) -> io::Result<()> {
        write!(self, "{{")?;
        write!(self, r#""name":{}"#, json!(name))?;
        write!(self, r#","assetId":"{hash}""#)?;
        if BITMAP_FORMATS.contains(&extension) {
            write!(
                self,
                r#","bitmapResolution":{}"#,
                json!(config.bitmap_resolution.unwrap_or(1))
            )?;
        }
        write!(self, r#","dataFormat":"{extension}""#)?;
        write!(self, r#","md5ext":"{hash}.{extension}""#)?;
        write!(self, "}}") // costume
    }

    pub fn costume(
        &mut self,
        config: &Config,
        fs: Rc<RefCell<dyn VFS>>,
        input: &Path,
        costume: &Asset,
        d: D,
    ) -> io::Result<()> {
        let path = input.join(&*costume.path);
        let hash = self
            .costumes
            .get(&costume.path)
            .cloned()
            .map(Ok::<_, io::Error>)
            .unwrap_or_else(|| {
                let mut fs = fs.borrow_mut();
                let mut file = match fs.read_file(&path) {
                    Ok(file) => file,
                    Err(error) => {
                        d.report_io_error(
                            error,
                            Some("costume files are always relative to the project directory"),
                            &costume.span,
                        );
                        return Ok(Default::default());
                    }
                };
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher)?;
                let hash: SmolStr = format!("{:x}", hasher.finalize()).into();
                self.costumes.insert(costume.path.clone(), hash.clone());
                Ok(hash)
            })?;
        let (_, extension) = costume.path.rsplit_once('.').unwrap_or_default();
        let extension = extension.to_lowercase();
        let extension = extension.as_str();
        if !(BITMAP_FORMATS.contains(&extension) || VECTOR_FORMATS.contains(&extension)) {
            d.report(
                DiagnosticKind::InvalidCostumeFormat {
                    extension: extension.into(),
                },
                &costume.span,
            );
        }
        self.costume_entry(config, &costume.name, &hash, extension)
    }
}
