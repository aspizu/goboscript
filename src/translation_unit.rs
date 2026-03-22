use std::{
    cell::RefCell,
    io,
    path::PathBuf,
    rc::Rc,
    str,
};

use fxhash::FxHashSet;
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use crate::{
    diagnostic::{
        Diagnostic,
        DiagnosticKind,
    },
    standard_library::StandardLibrary,
    vfs::VFS,
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Owner {
    Local,
    StandardLibrary,
}

fn replace_comments_with_whitespace(input: &mut [u8]) {
    let mut in_comment = false;

    for byte in input.iter_mut() {
        match byte {
            b'#' if !in_comment => {
                in_comment = true;
                *byte = b' ';
            }
            b'\n' => {
                in_comment = false;
                *byte = b' ';
            }
            _ if in_comment => {
                *byte = b' ';
            }
            _ => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// A section of a source file that is included in the translation unit.
/// This may be a section of the source file, or the entire source file.
pub struct Include {
    /// The range that the source code of the include is in the translation unit.
    pub unit_range: Span,
    // The range that the source code of the include is in the source file.
    pub source_range: Span,
    pub path: PathBuf,
    pub owner: Owner,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TranslationUnit {
    pub path: PathBuf,
    pub text: Vec<u8>,
    defines: FxHashSet<String>,
    includes: Vec<Include>,
    included: FxHashSet<PathBuf>,
    current_include: usize,
}

impl TranslationUnit {
    pub fn new(fs: Rc<RefCell<dyn VFS>>, path: PathBuf) -> io::Result<Self> {
        let mut text = fs.borrow_mut().read_to_vec(&path)?;
        if text.iter().last().is_none_or(|c| *c != b'\n') {
            text.push(b'\n');
        }
        let mut unit = Self {
            text,
            path,
            defines: Default::default(),
            includes: Default::default(),
            included: Default::default(),
            current_include: 0,
        };
        unit.includes.push(Include {
            unit_range: 0..unit.text.len(),
            source_range: 0..unit.text.len(),
            path: unit.path.clone(),
            owner: Owner::Local,
        });
        Ok(unit)
    }

    pub fn translate_position(&self, position: usize) -> (usize, &Include) {
        for include in &self.includes {
            debug_assert_eq!(include.unit_range.len(), include.source_range.len());
            if include.unit_range.contains(&position) {
                return (
                    include.source_range.start + (position - include.unit_range.start),
                    include,
                );
            }
        }
        panic!("invalid position {position} in {}", self.path.display());
    }
}

pub fn parse_translation_unit(
    unit: &mut TranslationUnit,
    fs: Rc<RefCell<dyn VFS>>,
    stdlib: &StandardLibrary,
    mut diagnostics: &mut Vec<Diagnostic>,
) {
    let mut i = 0;
    let mut skip_depth = 0;
    while i < unit.text.len() {
        if skip_depth > 0 {
            if i == 0 || unit.text[i - 1] == b'\n' {
                if unit.text[i..].starts_with(b"%if") {
                    unit.text[i] = b'#';
                    i += b"%if".len();
                    skip_depth += 1;
                } else if unit.text[i..].starts_with(b"%else") {
                    unit.text[i] = b'#';
                    i += b"%else".len();
                    if skip_depth == 1 {
                        skip_depth = 0;
                    }
                } else if unit.text[i..].starts_with(b"%endif") {
                    unit.text[i] = b'#';
                    i += b"%endif".len();
                    skip_depth -= 1;
                } else {
                    unit.text[i] = b'#';
                    i += 1;
                }
            } else {
                i += 1;
            }
        } else if (i == 0 || unit.text[i - 1] == b'\n') && unit.text[i] == b'%' {
            if unit.text[i..].starts_with(b"%include") {
                unit.text[i] = b'#';
                i += b"%include".len();
                while i < unit.text.len() && unit.text[i] == b' ' {
                    i += 1;
                }
                if i >= unit.text.len() {
                    continue;
                }
                let start = i;
                let j = unit.text[i..]
                    .iter()
                    .position(|c| *c == b'\n')
                    .map(|j| i + j + 1)
                    .unwrap_or(unit.text.len());
                let path = std::str::from_utf8(&unit.text[i..j])
                    .unwrap()
                    .trim()
                    .to_owned();
                let help = if path.ends_with(';') {
                    Some(
                        "pre-processor directives do not require a semicolon, try removing the `;`"
                            .into(),
                    )
                } else {
                    None
                };
                i = j;
                add_include_to_translation_unit(
                    unit,
                    path,
                    start..j,
                    i,
                    fs.clone(),
                    stdlib,
                    &mut diagnostics,
                    help,
                );
            } else if unit.text[i..].starts_with(b"%template") {
                unit.text[i..i + b"%template".len()].copy_from_slice(b"%define  ");
                i += b"%template".len();
                let begin = i;
                while i < unit.text.len() && !unit.text[i..].starts_with(b"%endtemplate") {
                    i += 1;
                }
                if unit.text[i..].starts_with(b"%endtemplate") {
                    replace_comments_with_whitespace(&mut unit.text[begin..i]);
                    unit.text[i] = b'#';
                    i += b"%endtemplate".len();
                }
            } else if unit.text[i..].starts_with(b"%define") {
                i += b"%define".len();
                while i < unit.text.len() && unit.text[i] == b' ' {
                    i += 1;
                }
                if i >= unit.text.len() {
                    continue;
                }
                let j = unit.text[i..]
                    .iter()
                    .position(|c| *c == b'\n')
                    .map(|j| i + j + 1)
                    .unwrap_or(unit.text.len());
                let x = unit.text[i..j]
                    .iter()
                    .position(|c| *c == b' ')
                    .map(|x| i + x)
                    .unwrap_or(j);
                let name = std::str::from_utf8(&unit.text[i..x])
                    .unwrap()
                    .trim()
                    .to_owned();
                unit.defines.insert(name);
                i = j;
            } else if unit.text[i..].starts_with(b"%undef") {
                i += b"%undef".len();
                while i < unit.text.len() && unit.text[i] == b' ' {
                    i += 1;
                }
                if i >= unit.text.len() {
                    continue;
                }
                let j = unit.text[i..]
                    .iter()
                    .position(|c| *c == b'\n')
                    .map(|j| i + j + 1)
                    .unwrap_or(unit.text.len());
                let name = std::str::from_utf8(&unit.text[i..j]).unwrap().trim();
                unit.defines.remove(name);
                i = j;
            } else if unit.text[i..].starts_with(b"%if") {
                unit.text[i] = b'#';
                i += b"%if".len();
                while i < unit.text.len() && unit.text[i] == b' ' {
                    i += 1;
                }
                if i >= unit.text.len() {
                    continue;
                }
                let inverted = if unit.text[i..].starts_with(b"not ") {
                    i += b"not ".len();
                    while i < unit.text.len() && unit.text[i] == b' ' {
                        i += 1;
                    }
                    true
                } else {
                    false
                };
                let j = unit.text[i..]
                    .iter()
                    .position(|c| *c == b'\n')
                    .map(|j| i + j + 1)
                    .unwrap_or(unit.text.len());
                let name = std::str::from_utf8(&unit.text[i..j]).unwrap().trim();
                if inverted == unit.defines.contains(name) {
                    skip_depth = 1;
                }
            } else if unit.text[i..].starts_with(b"%else") {
                unit.text[i] = b'#';
                i += b"%else".len();
                skip_depth = 1;
            } else if unit.text[i..].starts_with(b"%endif") {
                unit.text[i] = b'#';
                i += b"%endif".len();
            }
        } else {
            i += 1;
        }
    }
}

fn add_include_to_translation_unit(
    unit: &mut TranslationUnit,
    path: String,
    span: Span,
    start: usize,
    fs: Rc<RefCell<dyn VFS>>,
    stdlib: &StandardLibrary,
    diagnostics: &mut Vec<Diagnostic>,
    help: Option<String>,
) {
    let mut fs = fs.borrow_mut();

    while unit.includes[unit.current_include].unit_range.end < start {
        unit.current_include += 1;
    }

    let parent = unit.includes[unit.current_include].path.parent().unwrap();
    let (owner, path) = if let Some(path) = path.strip_prefix("std/") {
        (Owner::StandardLibrary, stdlib.path.join(path))
    } else if path.starts_with("./") || path.starts_with("../") {
        (Owner::Local, parent.join(path))
    } else {
        (Owner::Local, unit.path.parent().unwrap().join(path))
    };

    let mut path = path.normalize_lexically().unwrap();

    if unit.included.contains(&path) {
        return;
    }

    unit.included.insert(path.clone());

    if path.extension().is_none_or(|ext| ext != "gs") {
        path = path.with_added_extension("gs");
    }

    let mut buffer = match fs.read_to_vec(&path) {
        Ok(buffer) => buffer,
        Err(error) => {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::IOError {
                    error: error.to_string().into(),
                    help,
                },
                span,
            });
            return;
        }
    };

    if buffer.iter().last().is_none_or(|c| *c != b'\n') {
        buffer.push(b'\n');
    }

    unit.text.splice(start..start, buffer.iter().cloned());

    let current_include = unit.includes.remove(unit.current_include);

    // buffer before the include stmt
    let top_unit_range = current_include.unit_range.start..start;
    unit.includes.insert(
        unit.current_include,
        Include {
            unit_range: top_unit_range.clone(),
            source_range: current_include.source_range.start
                ..(current_include.source_range.start + top_unit_range.len()),
            path: current_include.path.clone(),
            owner: current_include.owner,
        },
    );

    // insert a new include in the middle
    unit.includes.insert(
        unit.current_include + 1,
        Include {
            unit_range: start..start + buffer.len(),
            source_range: 0..buffer.len(),
            path,
            owner,
        },
    );

    // buffer after the include stmt
    let bottom_unit_range = start..current_include.unit_range.end;
    unit.includes.insert(
        unit.current_include + 2,
        Include {
            unit_range: bottom_unit_range.clone(),
            source_range: (current_include.source_range.start + top_unit_range.len())
                ..(current_include.source_range.start
                    + top_unit_range.len()
                    + bottom_unit_range.len()),
            path: current_include.path,
            owner: current_include.owner,
        },
    );

    // adjust
    for include in &mut unit.includes[unit.current_include + 2..] {
        include.unit_range.start += buffer.len();
        include.unit_range.end += buffer.len();
    }

    unit.current_include += 1;
}
