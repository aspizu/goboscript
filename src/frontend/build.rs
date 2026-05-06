use std::{
    cell::RefCell,
    env,
    fs::File,
    io::BufWriter,
    path::PathBuf,
    rc::Rc,
};

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
    let project_name = canonical_input.file_name().unwrap().to_str().unwrap();
    let output = output.unwrap_or_else(|| input.join(format!("{project_name}.sb3")));
    let fs = Rc::new(RefCell::new(RealFS));
    let sb3 = Sb3::new(
        BufWriter::new(File::create(&output)?),
        fs.clone(),
        canonical_input.clone(),
    );
    build_impl(fs, canonical_input, sb3, None)
}
