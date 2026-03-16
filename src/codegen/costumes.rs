use std::{
    cell::RefCell,
    io,
    path::Path,
    rc::Rc,
};

use fxhash::FxHashMap;
use md5::{
    Digest,
    Md5,
};
use serde_json::json;

use super::sb3::{
    BITMAP_FORMATS,
    D,
    VECTOR_FORMATS,
};
use crate::{
    ast::Asset,
    config::Config,
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

mod hq;
use hq::costume_to_svg;

#[derive(Debug)]
pub struct CostumeWriter {
    pub costumes: FxHashMap<SmolStr, SmolStr>,
    pub hq_assets: FxHashMap<SmolStr, Vec<u8>>,
}

impl CostumeWriter {
    pub fn new() -> Self {
        Self {
            costumes: FxHashMap::default(),
            hq_assets: FxHashMap::default(),
        }
    }

    pub fn costume(
        &mut self,
        config: &Config,
        fs: Rc<RefCell<dyn VFS>>,
        input: &Path,
        costume: &Asset,
        d: D,
    ) -> io::Result<String> {
        let path = input.join(&*costume.path);
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
        if BITMAP_FORMATS.contains(&extension) {
            let hq_hash = self
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
                    let mut raw: Vec<u8> = Vec::new();
                    io::copy(&mut file, &mut raw)?;
                    let svg = costume_to_svg(&raw, extension);
                    let mut hasher = Md5::new();
                    hasher.update(&svg);
                    let hash: SmolStr = format!("{:x}", hasher.finalize()).into();
                    self.hq_assets.insert(hash.clone(), svg);
                    self.costumes.insert(costume.path.clone(), hash.clone());
                    Ok(hash)
                })?;
            return self.costume_entry(config, &costume.name, &hq_hash, "svg");
        }
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
        self.costume_entry(config, &costume.name, &hash, extension)
    }

    pub fn costume_entry(
        &mut self,
        config: &Config,
        name: &str,
        hash: &str,
        extension: &str,
    ) -> io::Result<String> {
        let mut result = String::new();
        result.push_str("{");
        result.push_str(&format!(r#""name":{}"#, json!(name)));
        result.push_str(&format!(r#","assetId":"{hash}""#));
        if BITMAP_FORMATS.contains(&extension) {
            result.push_str(&format!(
                r#","bitmapResolution":{}"#,
                json!(config.bitmap_resolution.unwrap_or(1))
            ));
        }
        result.push_str(&format!(r#","dataFormat":"{extension}""#));
        result.push_str(&format!(r#","md5ext":"{hash}.{extension}""#));
        result.push('}');
        Ok(result)
    }
}
