use std::{
    cell::RefCell,
    env,
    fs::{
        File,
        OpenOptions,
    },
    io::{
        self,
        BufWriter,
    },
    path::{
        Path,
        PathBuf,
    },
    rc::Rc,
};

use crate::{
    codegen::build::build_impl,
    diagnostic::Artifact,
    vfs::RealFS,
};

pub fn build(input: Option<PathBuf>, output: Option<PathBuf>) -> anyhow::Result<Artifact> {
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    let canonical_input = input.canonicalize()?;
    let project_name = canonical_input.file_name().unwrap().to_str().unwrap();
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let fs = Rc::new(RefCell::new(RealFS));
    let file = BufWriter::new(open_output(&output)?);
    build_impl(fs, canonical_input, file, None)
}

fn open_output(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;

        use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    #[cfg(not(any(unix, windows)))]
    return Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "opening output without following symlinks is unsupported",
    ));

    let file = options.open(path)?;
    // Inspect and truncate the same handle; never check the path before opening it.
    #[cfg(windows)]
    if file.metadata()?.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "output path is a symlink",
        ));
    }
    file.set_len(0)?;
    Ok(file)
}
