use std::{
    cell::RefCell,
    env,
    fs::File,
    io::BufWriter,
    path::PathBuf,
    rc::Rc,
};

use anyhow::bail;

use crate::{
    codegen::{
        build::build_impl,
        sb3::Sb3,
    },
    diagnostic::Artifact,
    vfs::RealFS,
};

pub fn build(input: Option<PathBuf>, output: Option<PathBuf>) -> anyhow::Result<Artifact> {
    let input = input.unwrap_or_else(|| env::current_dir().unwrap());
    let canonical_input = input.canonicalize()?;
    let Some(project_name) = canonical_input.file_name().and_then(|name| name.to_str()) else {
        bail!(
            "{} is not a valid project directory",
            canonical_input.display()
        );
    };
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let fs = Rc::new(RefCell::new(RealFS));
    let sb3 = Sb3::new(
        BufWriter::new(File::create(&output)?),
        fs.clone(),
        canonical_input.clone(),
    );
    build_impl(fs, canonical_input, sb3, None)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::build;

    #[test]
    fn test_build_root_dir_errors_without_panic() {
        match build(Some(PathBuf::from("/")), None) {
            Ok(_) => panic!("building the filesystem root should return an error"),
            Err(err) => assert!(err.to_string().contains("not a valid project directory")),
        }
    }
}
