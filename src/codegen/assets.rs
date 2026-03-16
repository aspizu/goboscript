use std::{
    cell::RefCell,
    io::{
        self,
        Write,
    },
    path::PathBuf,
    rc::Rc,
};

use base64::{
    prelude::BASE64_URL_SAFE,
    Engine,
};
use fxhash::{
    FxHashMap,
    FxHashSet,
};
use md5::{
    Digest,
    Md5,
};
use zip::write::SimpleFileOptions;

use crate::{
    ast::Asset,
    codegen::sb3::{
        Sb3,
        D,
    },
    misc::SmolStr,
    vfs::VFS,
};

#[derive(Debug, Default)]
pub struct AssetObject {
    pub hash: String,
    pub extension: String,
    pub content: Vec<u8>,
}

pub struct AssetObjectStore {
    store: FxHashMap<SmolStr, AssetObject>,
    fs: Rc<RefCell<dyn VFS>>,
    input: PathBuf,
}

impl AssetObjectStore {
    pub fn new(input: PathBuf, fs: Rc<RefCell<dyn VFS>>) -> Self {
        Self {
            store: FxHashMap::default(),
            fs,
            input,
        }
    }

    pub fn load(&mut self, asset: &Asset, d: D) -> &AssetObject {
        self.store.entry(asset.path.clone()).or_insert_with(|| {
            let mut fs = self.fs.borrow_mut();
            let mut content = match fs.read_to_vec(&self.input.join(&*asset.path)) {
                Ok(content) => content,
                Err(error) => {
                    d.report_io_error(
                        error,
                        Some("costume/sound files are always relative to the project directory"),
                        &asset.span,
                    );
                    return Default::default();
                }
            };
            let mut extension = asset.path.rsplit_once('.').unwrap_or_default().1.to_owned();
            if let Some(feat) = &asset.feature {
                if feat == "hq" {
                    feature_hq(&mut extension, &mut content).unwrap();
                }
            }

            let mut hasher = Md5::new();
            hasher.update(&content);
            let hash = format!("{:x}", hasher.finalize()).to_string();
            AssetObject {
                hash,
                content,
                extension,
            }
        })
    }

    pub fn get_objects(&self) -> impl Iterator<Item = &AssetObject> {
        self.store.values()
    }
}

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn assets(&mut self) -> io::Result<()> {
        let mut added = FxHashSet::default();
        for object in self.asset_object_store.get_objects() {
            if added.contains(&&*object.hash) {
                continue;
            }
            added.insert(object.hash.as_str());
            self.zip.start_file(
                format!("{}.{}", object.hash, object.extension),
                SimpleFileOptions::default(),
            )?;
            self.zip.write_all(&object.content)?;
        }
        Ok(())
    }
}

fn feature_hq(extension: &mut String, file: &mut Vec<u8>) -> io::Result<()> {
    let b64 = BASE64_URL_SAFE.encode(file.as_slice());
    file.clear();
    file.extend(
        br#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="480" height="360" viewBox="0 0 480 360">"#,
    );
    write!(
        file,
        r#"<image width="480" height="360" xlink:href="data:image/{};base64,{}"/>"#,
        extension, b64
    )?;
    file.extend(br#"</svg>"#);
    *extension = "svg".to_owned();
    Ok(())
}
