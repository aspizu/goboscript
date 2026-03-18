use std::{
    cell::RefCell,
    io::{
        self,
        Write,
    },
    path::PathBuf,
    rc::Rc,
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
    pub input: PathBuf,
}

impl AssetObjectStore {
    pub fn new(input: PathBuf, fs: Rc<RefCell<dyn VFS>>) -> Self {
        Self {
            store: FxHashMap::default(),
            fs,
            input,
        }
    }

    /// Preload a batch of asset paths in parallel (native only).
    /// Must be called before `load()` for best effect.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn preload(&mut self, paths: &[SmolStr]) {
        use rayon::prelude::*;

        let input = &self.input;
        let results: Vec<(SmolStr, AssetObject)> = paths
            .par_iter()
            .filter(|path| !self.store.contains_key(*path))
            .filter_map(|path| {
                let full_path = input.join(&**path);
                let content = match std::fs::read(&full_path) {
                    Ok(c) => c,
                    Err(_) => return None,
                };
                let extension = path
                    .rsplit_once('.')
                    .unwrap_or_default()
                    .1
                    .to_lowercase();
                let mut hasher = Md5::new();
                hasher.update(&content);
                let hash = format!("{:x}", hasher.finalize());
                Some((path.clone(), AssetObject {
                    hash,
                    content,
                    extension,
                }))
            })
            .collect();

        for (path, obj) in results {
            self.store.entry(path).or_insert(obj);
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
