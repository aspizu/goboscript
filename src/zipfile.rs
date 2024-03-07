use std::io::{self, Seek, SeekFrom, Write};

use crc32fast::Hasher;

struct File {
    name: String,
    crc32: u32,
    size: u64,
    header_start: u64,
}

pub struct ZipFile<T>
where
    T: Write + Seek,
{
    file: T,
    files: Vec<File>,
    crc32: Hasher,
    writing_file: Option<File>,
}

impl<T> ZipFile<T>
where
    T: Write + Seek,
{
    pub fn new(file: T) -> ZipFile<T> {
        ZipFile {
            file,
            files: Default::default(),
            crc32: Hasher::new(),
            writing_file: None,
        }
    }

    pub fn end_zip(&mut self) -> io::Result<()> {
        if self.writing_file.is_some() {
            self.end_file()?;
        }
        let begin = self.file.stream_position()?;
        for file in &self.files {
            Self::file_header(
                &mut self.file,
                &file.name,
                file.crc32,
                file.size as u32,
                true,
                file.header_start as u32,
            )?;
        }
        let end = self.file.stream_position()?;
        let size = (end - begin) as u32;
        Self::end_of_central_directory(
            &mut self.file,
            self.files.len() as u16,
            size,
            begin as u32,
        )?;
        Ok(())
    }

    pub fn begin_file(&mut self, name: &str) -> io::Result<()> {
        if self.writing_file.is_some() {
            self.end_file()?;
        }
        let header_start = self.file.stream_position()?;
        Self::file_header(&mut self.file, name, 0, 0, false, 0)?;
        self.writing_file = Some(File {
            name: name.to_owned(),
            size: 0,
            crc32: 0,
            header_start,
        });
        self.crc32.reset();
        Ok(())
    }

    fn end_file(&mut self) -> io::Result<()> {
        let mut file = self.writing_file.take().unwrap();
        let crc32 = self.crc32.clone().finalize();
        file.crc32 = crc32;
        Self::set_file_header(&mut self.file, file.header_start, crc32, file.size as u32)?;
        self.file.seek(SeekFrom::End(0))?;
        self.files.push(file);
        Ok(())
    }

    fn set_file_header(file: &mut T, header_start: u64, crc32: u32, size: u32) -> io::Result<()> {
        file.seek(io::SeekFrom::Start(header_start + 14))?;
        file.write_all(&crc32.to_le_bytes())?;
        file.write_all(&size.to_le_bytes())?;
        file.write_all(&size.to_le_bytes())?;
        Ok(())
    }

    fn file_header(
        file: &mut T,
        name: &str,
        crc32: u32,
        size: u32,
        is_directory_header: bool,
        offset: u32,
    ) -> io::Result<()> {
        // local file header signature
        if is_directory_header {
            file.write_all(&0x02014b50_u32.to_le_bytes())?;
            // version made by
            file.write_all(&0x031e_u16.to_le_bytes())?;
        } else {
            file.write_all(&0x04034b50_u32.to_le_bytes())?;
        }
        // version needed to extract
        file.write_all(&0x031e_u16.to_le_bytes())?;
        // general purpose bit flag
        file.write_all(&0b_0000_0000_0000_0000_u16.to_be_bytes())?;
        // compression method
        file.write_all(&0_u16.to_le_bytes())?;
        // last mod file time
        file.write_all(&0_u16.to_le_bytes())?;
        // last mod file date
        file.write_all(&0_u16.to_le_bytes())?;
        // crc-32
        file.write_all(&crc32.to_le_bytes())?;
        // compressed size
        file.write_all(&size.to_le_bytes())?;
        // uncompressed size
        file.write_all(&size.to_le_bytes())?;
        // file name length
        file.write_all(&(name.len() as u16).to_le_bytes())?;
        // extra field length
        file.write_all(&0_u16.to_le_bytes())?;
        if is_directory_header {
            // file comment length
            file.write_all(&0_u16.to_le_bytes())?;
            // disk number start
            file.write_all(&0_u16.to_le_bytes())?;
            // internal file attributes
            file.write_all(&0_u16.to_le_bytes())?;
            // external file attributes
            file.write_all(&0x81a40000_u32.to_le_bytes())?;
            // relative offset of local header
            file.write_all(&offset.to_le_bytes())?;
        }
        // file name
        file.write_all(name.as_bytes())?;
        // extra field
        // ...
        Ok(())
    }

    fn end_of_central_directory(
        file: &mut T,
        files: u16,
        size: u32,
        offset: u32,
    ) -> io::Result<()> {
        // end of central dir signature
        file.write_all(&0x06054b50_u32.to_le_bytes())?;
        // number of this disk
        file.write_all(&0_u16.to_le_bytes())?;
        // number of the disk with the start of the central directory
        file.write_all(&0_u16.to_le_bytes())?;
        // total number of entries in the central directory on this disk
        file.write_all(&files.to_le_bytes())?;
        // total number of entries in the central directory
        file.write_all(&files.to_le_bytes())?;
        // size of the central directory
        file.write_all(&size.to_le_bytes())?;
        // offset of start of central directory with respect to the starting disk number
        file.write_all(&offset.to_le_bytes())?;
        // .ZIP file comment length
        file.write_all(&0_u16.to_le_bytes())?;
        // .ZIP file comment
        // ...
        Ok(())
    }
}

impl<T> Write for ZipFile<T>
where
    T: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(file) = &mut self.writing_file {
            self.crc32.update(buf);
            file.size += buf.len() as u64;
        }
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
