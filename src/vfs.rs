use std::{
    io::{
        self,
        Cursor,
    },
    path::{
        Path,
        PathBuf,
    },
};

use fxhash::FxHashMap;
use serde::{
    Deserialize,
    Serialize,
};

pub trait VFS {
    fn read_dir(&mut self, path: &Path) -> io::Result<Vec<PathBuf>>;
    fn read_file<'a>(&'a mut self, path: &Path) -> io::Result<Box<dyn io::Read + 'a>>;
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;

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

pub struct RealFS;

impl Default for RealFS {
    fn default() -> Self {
        Self::new()
    }
}

impl RealFS {
    pub fn new() -> Self {
        Self {}
    }
}

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
}

mod base64 {
    use serde::{
        Deserialize,
        Deserializer,
        Serialize,
        Serializer,
    };

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    #[serde(with = "base64")]
    pub inner: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct MemFS {
    files: FxHashMap<String, Data>,
}

impl VFS for MemFS {
    fn read_dir<'a>(&mut self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let path = path.to_str().unwrap();
        let mut entries = Vec::new();
        for (key, _) in self.files.iter() {
            if key.starts_with(path)
                && key.chars().filter(|c| *c == '/').count()
                    == path.chars().filter(|c| *c == '/').count()
            {
                entries.push(PathBuf::from(key));
            }
        }
        Ok(entries)
    }

    fn read_file<'a>(&'a mut self, path: &Path) -> io::Result<Box<dyn io::Read + 'a>> {
        let data = self.files.get(path.to_str().unwrap()).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )
        })?;
        let cursor = Cursor::new(&data.inner);
        Ok(Box::new(cursor))
    }

    fn is_dir(&self, path: &Path) -> bool {
        let path = path.to_str().unwrap();
        self.files
            .keys()
            .any(|key| key.starts_with(path) && key != path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.files.contains_key(path.to_str().unwrap()) && !self.is_dir(path)
    }
}
