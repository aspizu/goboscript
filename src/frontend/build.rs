use std::{
    cell::RefCell,
    env,
    fs::File,
    io::BufWriter,
    path::PathBuf,
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
    let file = BufWriter::new(File::create(&output)?);
    build_impl(fs, canonical_input, file, None)
}
