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
    ast::Value,
    diagnostic::DiagnosticKind,
    misc::SmolStr,
    vfs::VFS,
};

pub fn read_list(
    fs: Rc<RefCell<dyn VFS>>,
    input: &Path,
    path: &SmolStr,
) -> Result<Vec<Value>, DiagnosticKind> {
    let (_, _ext) = path.rsplit_once('.').unwrap_or_default();
    let mut fs = fs.borrow_mut();
    let mut file = fs.read_file(&input.join(&**path)).map_err(|err| {
        DiagnosticKind::io_error(
            err,
            Some("list files are always relative to the project directory"),
        )
    })?;
    read_list_text(&mut file).map_err(|err| DiagnosticKind::io_error(err, None))
}

fn read_list_text(file: &mut Box<dyn io::Read + '_>) -> Result<Vec<Value>, io::Error> {
    let file = BufReader::new(file);
    Ok(file.lines().map_while(Result::ok).map(Into::into).collect())
}
