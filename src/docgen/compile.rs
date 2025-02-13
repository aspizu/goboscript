use std::{
    fs,
    path::PathBuf,
};

use fxhash::FxHashMap;
use serde::Serialize;
use walkdir::WalkDir;

use super::docstring::{
    find_docstrings,
    Symbol,
};
use crate::{
    ast::*,
    diagnostic::SpriteDiagnostics,
    misc::SmolStr,
    parser,
    standard_library::StandardLibrary,
};

#[derive(Debug, Serialize)]
pub struct File {
    pub path: SmolStr,
    pub docstrings: FxHashMap<Symbol, SmolStr>,
    pub funcs: FxHashMap<SmolStr, Func>,
    pub procs: FxHashMap<SmolStr, Proc>,
    pub enums: FxHashMap<SmolStr, Enum>,
    pub structs: FxHashMap<SmolStr, Struct>,
    pub vars: FxHashMap<SmolStr, Var>,
    pub lists: FxHashMap<SmolStr, List>,
}
pub fn compile(
    input: PathBuf,
    stdlib: &StandardLibrary,
) -> anyhow::Result<FxHashMap<SmolStr, File>> {
    let mut files: FxHashMap<SmolStr, File> = Default::default();
    for entry in WalkDir::new(&input) {
        let entry = entry?;
        let path = entry.path();
        let relpath = path.strip_prefix(&input).unwrap();
        if path.is_file() && path.extension().is_some_and(|extension| extension == "gs") {
            let text = fs::read_to_string(path)?;
            let docstrings = find_docstrings(&text);
            let mut diagnostics = SpriteDiagnostics::new(path.to_path_buf(), stdlib);
            // TODO: disable file inclusion for document generation to avoid
            // dealing with backpack
            let sprite = parser::parse(&diagnostics.translation_unit)
                .map_err(|err| {
                    diagnostics.diagnostics.push(err);
                })
                .unwrap_or_default();
            files.insert(
                relpath.to_string_lossy().into(),
                File {
                    path: relpath.to_string_lossy().into(),
                    docstrings,
                    funcs: sprite.funcs,
                    procs: sprite.procs,
                    enums: sprite.enums,
                    structs: sprite.structs,
                    vars: sprite.vars,
                    lists: sprite.lists,
                },
            );
        }
    }
    Ok(files)
}
