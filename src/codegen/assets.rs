use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
};

use fxhash::FxHashMap;
use md5::{
    Digest,
    Md5,
};

use crate::{
    ast::Asset,
    codegen::sb3::D,
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
            let content = match fs.read_to_vec(&self.input.join(&*asset.path)) {
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
            let extension = asset
                .path
                .rsplit_once('.')
                .unwrap_or_default()
                .1
                .to_lowercase();

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
