use std::{
    borrow::Cow,
    io::{
        self,
        Cursor,
        Read,
    },
    path::{
        Path,
        PathBuf,
    },
};

use fxhash::FxHashMap;
use glob::{
    glob,
    MatchOptions,
    Pattern,
};
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use crate::misc::base64;

pub trait VFS {
    fn read_dir(&mut self, path: &Path) -> io::Result<Vec<PathBuf>>;
    fn read_file<'a>(&'a mut self, path: &Path) -> io::Result<Box<dyn io::Read + 'a>>;
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
    fn glob(&mut self, pattern: &str) -> io::Result<Vec<PathBuf>>;

    fn read_to_string(&mut self, path: &Path) -> io::Result<String> {
        let mut file = self.read_file(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn read_to_vec(&mut self, path: &Path) -> io::Result<Vec<u8>> {
        let mut file = self.read_file(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
}

#[derive(Default)]
pub struct RealFS;

impl VFS for RealFS {
    fn read_dir(&mut self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in path.read_dir()? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }

    fn read_file<'a>(&'a mut self, path: &Path) -> io::Result<Box<dyn io::Read + 'a>> {
        let file = std::fs::File::open(path)?;
        Ok(Box::new(file))
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn glob(&mut self, pattern: &str) -> io::Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        let paths =
            glob(pattern).map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.msg))?;
        for entry in paths {
            let path = entry.map_err(|err| io::Error::new(err.error().kind(), err.to_string()))?;
            entries.push(path);
        }
        Ok(entries)
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct Data {
    #[serde(with = "base64")]
    pub inner: Vec<u8>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemFS {
    files: FxHashMap<String, Data>,
}

impl VFS for MemFS {
    fn read_dir(&mut self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let path_str = path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8 in path"))?;

        // Normalize path: empty string or "/" means root
        let normalized_path = match path_str {
            "" | "." | "/" => "",
            s if s.ends_with('/') => s,
            s => &(s.to_owned() + "/"),
        };

        let mut entries = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for key in self.files.keys() {
            if let Some(remainder) = key.strip_prefix(normalized_path) {
                if let Some(pos) = remainder.find('/') {
                    let entry = &remainder[..pos];
                    if seen.insert(entry) {
                        entries.push(path.join(entry));
                    }
                } else if seen.insert(remainder) {
                    entries.push(path.join(remainder));
                }
            }
        }

        Ok(entries)
    }

    fn read_file<'a>(&'a mut self, path: &Path) -> io::Result<Box<dyn io::Read + 'a>> {
        let path_str = path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8 in path"))?;

        let data = self.files.get(path_str).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )
        })?;

        Ok(Box::new(Cursor::new(&data.inner)))
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.to_str().is_some_and(|path_str| {
            let normalized_path = if path_str.ends_with('/') {
                path_str.to_string()
            } else {
                format!("{}/", path_str)
            };
            self.files
                .keys()
                .any(|key| key.starts_with(&normalized_path))
        })
    }

    fn is_file(&self, path: &Path) -> bool {
        path.to_str()
            .is_some_and(|path_str| self.files.contains_key(path_str))
    }

    fn glob(&mut self, pattern: &str) -> io::Result<Vec<PathBuf>> {
        let pattern = Pattern::new(&normalize_glob_path(pattern))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.msg))?;
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };
        let mut entries = Vec::new();
        for key in self.files.keys() {
            if pattern.matches_with(&normalize_glob_path(key), options) {
                entries.push(PathBuf::from(key));
            }
        }
        Ok(entries)
    }
}

fn normalize_glob_path(path: &str) -> Cow<'_, str> {
    if path.contains('\\') {
        Cow::Owned(path.replace('\\', "/"))
    } else {
        Cow::Borrowed(path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{
        Path,
        PathBuf,
    };

    use super::*;

    #[test]
    fn test_memfs_read_write_file() {
        let mut memfs = MemFS {
            files: FxHashMap::default(),
        };
        let path = Path::new("test_file.txt");
        let content = b"Hello, MemFS!".to_vec();

        // Write file
        memfs.files.insert(
            path.to_str().unwrap().to_string(),
            Data {
                inner: content.clone(),
            },
        );

        // Read file
        let mut file = memfs.read_file(path).expect("File should exist");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Read should succeed");

        assert_eq!(buffer, content);
    }

    #[test]
    fn test_memfs_is_file() {
        let mut memfs = MemFS {
            files: FxHashMap::default(),
        };
        let path = Path::new("test_file.txt");
        memfs
            .files
            .insert(path.to_str().unwrap().to_string(), Data { inner: vec![] });

        assert!(memfs.is_file(path));
        assert!(!memfs.is_file(Path::new("non_existent.txt")));
    }

    #[test]
    fn test_memfs_is_dir() {
        let mut memfs = MemFS {
            files: FxHashMap::default(),
        };
        memfs
            .files
            .insert("dir/file.txt".to_string(), Data { inner: vec![] });

        assert!(memfs.is_dir(Path::new("dir/")));
        assert!(!memfs.is_dir(Path::new("dir/file.txt")));
        assert!(!memfs.is_dir(Path::new("non_existent_dir/")));
    }

    #[test]
    fn test_memfs_read_dir() {
        let mut memfs = MemFS {
            files: FxHashMap::default(),
        };
        memfs
            .files
            .insert("rootfile1.txt".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("dir/file1.txt".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("dir/file2.txt".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("dir/subdir/file3.txt".to_string(), Data { inner: vec![] });

        let entries = memfs
            .read_dir(Path::new("dir"))
            .expect("Read dir should succeed");
        let entry_names: Vec<_> = entries
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        assert!(entry_names.contains(&"file1.txt".to_string()));
        assert!(entry_names.contains(&"file2.txt".to_string()));
        assert!(entry_names.contains(&"subdir".to_string()));
    }

    #[test]
    fn test_memfs_glob() {
        let mut memfs = MemFS {
            files: FxHashMap::default(),
        };
        memfs
            .files
            .insert("assets/a.png".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("assets/b.jpg".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("assets/sub/c.png".to_string(), Data { inner: vec![] });
        memfs
            .files
            .insert("root.txt".to_string(), Data { inner: vec![] });

        let mut pngs = memfs.glob("assets/*.png").expect("glob should succeed");
        pngs.sort();
        assert_eq!(pngs, vec![PathBuf::from("assets/a.png")]);

        let mut deep_pngs = memfs.glob("assets/**/*.png").expect("glob should succeed");
        deep_pngs.sort();
        assert_eq!(
            deep_pngs,
            vec![
                PathBuf::from("assets/a.png"),
                PathBuf::from("assets/sub/c.png")
            ]
        );
    }
}
