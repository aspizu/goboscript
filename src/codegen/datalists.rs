use std::{
    cell::RefCell,
    io::{
        self,
        BufRead,
        BufReader,
    },
    path::Path,
    rc::Rc,
};

use crate::{
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

pub fn read_list(
    fs: Rc<RefCell<dyn VFS>>,
    input: &Path,
    path: &SmolStr,
) -> Result<Vec<SmolStr>, DiagnosticKind> {
    let (_, ext) = path.rsplit_once('.').unwrap_or_default();
    let mut fs = fs.borrow_mut();
    let mut file = fs
        .read_file(&input.join(&**path))
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
