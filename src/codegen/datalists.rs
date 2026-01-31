use std::{
    cell::RefCell,
    io::{
        self,
        BufRead,
        BufReader,
    },
    path::{
        Path,
        PathBuf,
    },
    rc::Rc,
};

use crate::{
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

fn resolve_path(base_path: Option<&Path>, path: &str, input: &Path) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("./") {
        if let Some(parent) = base_path.and_then(|p| p.parent()) {
            return parent.join(stripped);
        }
        return input.join(stripped);
    } else if let Some(stripped) = path.strip_prefix("../") {
        if let Some(parent) = base_path.and_then(|p| p.parent()).and_then(|p| p.parent()) {
            return parent.join(stripped);
        }
        return input.join(stripped);
    }
    // No prefix: relative to sprite directory (or input for stage)
    if let Some(parent) = base_path.and_then(|p| p.parent()) {
        return parent.join(path);
    }
    input.join(path)
}

pub fn read_list(
    fs: Rc<RefCell<dyn VFS>>,
    input: &Path,
    path: &SmolStr,
    base_path: Option<&Path>,
) -> Result<Vec<SmolStr>, DiagnosticKind> {
    let (_, ext) = path.rsplit_once('.').unwrap_or_default();
    let resolved_path = resolve_path(base_path, path, input);
    let mut fs = fs.borrow_mut();
    let mut file = fs
        .read_file(&resolved_path)
        .map_err(|err| DiagnosticKind::IOError(err.to_string().into()))?;
    match ext {
        _ => read_list_text(&mut file),
    }
    .map_err(|err| DiagnosticKind::IOError(err.to_string().into()))
}

fn read_list_text(file: &mut Box<dyn io::Read + '_>) -> Result<Vec<SmolStr>, io::Error> {
    let file = BufReader::new(file);
    Ok(file.lines().into_iter().flatten().map(Into::into).collect())
}
