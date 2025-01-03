use std::{
    fs::File,
    io::{self, Cursor, Write},
    path::Path,
};

use md5::{Digest, Md5};
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};

use super::sb3::Sb3;

impl<T> Sb3<T>
where T: io::Write + io::Seek
{
    pub fn srcpkg(&mut self, input: &Path, output: &Path) -> io::Result<()> {
        let zipped = create_zipped_srcpkg(input, output)?;
        let header = include_bytes!("srcpkg.svg");
        let mut file: Vec<u8> = Vec::with_capacity(header.len());
        file.write_all(b"<!--")?;
        write!(file, "{:08x}", zipped.len())?;
        let mut b64 =
            base64::write::EncoderWriter::new(file, &base64::engine::general_purpose::STANDARD);
        b64.write_all(&zipped)?;
        let mut file = b64.finish()?;
        file.write_all(b"-->")?;
        file.extend_from_slice(header);
        let mut hasher = Md5::new();
        hasher.update(&file);
        let hash = format!("{:x}", hasher.finalize());
        self.srcpkg_hash = Some(hash);
        self.srcpkg = Some(file);
        Ok(())
    }

    pub fn srcpkg_entry(&mut self) -> io::Result<()> {
        let hash = self.srcpkg_hash.take().unwrap();
        self.costume_entry("__srcpkg__", &hash, "svg")?;
        self.srcpkg_hash = Some(hash);
        Ok(())
    }
}

fn create_zipped_srcpkg(input: &Path, output: &Path) -> io::Result<Vec<u8>> {
    let output = output
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join(output.file_name().unwrap());
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for entry in WalkDir::new(input).into_iter().flatten() {
        if !entry.metadata().is_ok_and(|metadata| metadata.is_file()) {
            continue;
        }
        if entry.path().canonicalize().unwrap() == output {
            continue;
        }
        zip.start_file(entry.path().to_string_lossy(), SimpleFileOptions::default())?;
        let mut file = File::open(entry.path())?;
        std::io::copy(&mut file, &mut zip)?;
    }
    Ok(zip.finish()?.into_inner())
}
