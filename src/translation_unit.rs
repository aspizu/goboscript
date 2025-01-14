use std::{
    fs::{
        self,
        File,
    },
    io::Read,
    path::PathBuf,
    str,
};

use fxhash::FxHashSet;
use logos::Span;

use crate::diagnostic::{
    Diagnostic,
    DiagnosticKind,
};

#[derive(Debug)]
/// A section of a source file that is included in the translation unit.
/// This may be a section of the source file, or the entire source file.
pub struct Include {
    /// The range that the source code of the include is in the translation unit.
    pub unit_range: Span,
    // The range that the source code of the include is in the source file.
    pub source_range: Span,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct TranslationUnit {
    path: PathBuf,
    text: Vec<u8>,
    defines: FxHashSet<String>,
    includes: Vec<Include>,
    included: FxHashSet<String>,
    current_include: usize,
}

impl TranslationUnit {
    pub fn new(path: PathBuf) -> Self {
        let text = fs::read(&path).unwrap();
        let mut instance = Self {
            text,
            path,
            defines: Default::default(),
            includes: Default::default(),
            included: Default::default(),
            current_include: 0,
        };
        instance.includes.push(Include {
            unit_range: 0..instance.text.len(),
            source_range: 0..instance.text.len(),
            path: Some(instance.path.clone()),
        });
        instance
    }

    pub fn pre_process(&mut self) -> Result<(), Diagnostic> {
        self.parse(0)
    }

    pub fn get_text(&self) -> &str {
        str::from_utf8(&self.text).unwrap()
    }

    fn parse(&mut self, begin: usize) -> Result<(), Diagnostic> {
        let mut comment = 0;
        let mut i = begin;
        while i < self.text.len() {
            if 0 < comment {
                if self.text[i..].starts_with(b"\n%") {
                    i += b"\n%".len();
                    self.text[i - 1] = b'#';
                    if self.text[i..].starts_with(b"if") {
                        comment += 1;
                    } else if self.text[i..].starts_with(b"endif") {
                        comment -= 1;
                    }
                } else if self.text[i..].starts_with(b"\n") {
                    i += 1;
                    if i < self.text.len() {
                        self.text[i] = b'#';
                    }
                } else {
                    i += 1;
                }
            } else {
                let mut begin = false;
                if self.text[i..].starts_with(b"\n%") {
                    i += b"\n%".len();
                    begin = true;
                } else if i == 0 && self.text.starts_with(b"%") {
                    i += b"%".len();
                    begin = true;
                }
                if begin {
                    if self.text[i..].starts_with(b"include") {
                        self.text[i - 1] = b'#';
                        i += b"include".len();
                        let path = self.text[i..]
                            .split(|c| *c == b'\n' || *c == b'\r')
                            .next()
                            .unwrap();
                        let path_span = i..(i + path.len());
                        i += path.len();
                        if self.text[i..].starts_with(b"\r") {
                            i += 1;
                        }
                        if self.text[i..].starts_with(b"\n") {
                            i += 1;
                        }
                        let path = str::from_utf8(path).unwrap().trim().to_owned();
                        if !self.included.contains(&path) {
                            self.include(&path, path_span, i)?;
                            self.included.insert(path);
                        }
                    } else if self.text[i..].starts_with(b"define") {
                        i += b"define".len();
                        let name = self.text[i..]
                            .split(|c| *c == b'\n' || *c == b'\r')
                            .next()
                            .unwrap();
                        i += name.len();
                        if self.text[i..].starts_with(b"\r") {
                            i += 1;
                        }
                        let name = str::from_utf8(name).unwrap().trim();
                        self.defines.insert(name.to_string());
                    } else if self.text[i..].starts_with(b"undef") {
                        i += b"undef".len();
                        let name = self.text[i..]
                            .split(|c| *c == b'\n' || *c == b'\r')
                            .next()
                            .unwrap();
                        i += name.len();
                        if self.text[i..].starts_with(b"\r") {
                            i += 1;
                        }
                        let name = str::from_utf8(name).unwrap().trim();
                        self.defines.remove(name);
                    } else if self.text[i..].starts_with(b"if") {
                        self.text[i - 1] = b'#';
                        i += b"if".len();
                        let mut invert = false;
                        if self.text[i..].starts_with(b" not ") {
                            i += b" not ".len();
                            invert = true;
                        }
                        let name = self.text[i..]
                            .split(|c| *c == b'\n' || *c == b'\r')
                            .next()
                            .unwrap();
                        i += name.len();
                        if self.text[i..].starts_with(b"\r") {
                            i += 1;
                        }
                        let name = str::from_utf8(name).unwrap().trim();
                        if self.defines.contains(name) == invert {
                            comment = 1;
                        }
                    } else if self.text[i..].starts_with(b"endif") {
                        self.text[i - 1] = b'#';
                        i += b"endif".len();
                    }
                } else {
                    i += 1;
                }
            }
        }
        Ok(())
    }

    fn include(&mut self, path: &str, path_span: Span, begin: usize) -> Result<(), Diagnostic> {
        let mut buffer = vec![];
        let (file, path): (&[u8], Option<PathBuf>) = if let Some(path) = path.strip_prefix("std/") {
            let path = path.strip_suffix(".gs").unwrap_or(path);
            let file = match path {
                "algo" => include_bytes!("../std/algo.gs").as_slice(),
                "emoji" => include_bytes!("../std/emoji.gs").as_slice(),
                "math" => include_bytes!("../std/math.gs").as_slice(),
                "string" => include_bytes!("../std/string.gs").as_slice(),
                _ => todo!("{path}"),
            };
            (file, None)
        } else {
            let mut path = self.path.parent().unwrap().join(path);
            path.set_extension("gs");
            let mut file = File::open(&path).map_err(|error| Diagnostic {
                kind: DiagnosticKind::IOError(error),
                span: path_span,
            })?;
            file.read_to_end(&mut buffer).unwrap();
            (&buffer, Some(path))
        };
        self.text.splice(begin..begin, file.iter().cloned());

        // split current include into two parts

        let current_include = self.includes.remove(self.current_include);

        // file before the include stmt
        let top_unit_range = current_include.unit_range.start..begin;
        self.includes.insert(
            self.current_include,
            Include {
                unit_range: top_unit_range.clone(),
                source_range: current_include.source_range.start
                    ..(current_include.source_range.start + top_unit_range.len()),
                path: current_include.path.clone(),
            },
        );

        // insert a new include in the middle
        self.includes.insert(
            self.current_include + 1,
            Include {
                unit_range: begin..begin + file.len(),
                source_range: 0..file.len(),
                path,
            },
        );

        // file after the include stmt
        let bottom_unit_range = begin..current_include.unit_range.end;
        self.includes.insert(
            self.current_include + 2,
            Include {
                unit_range: bottom_unit_range.clone(),
                source_range: (current_include.source_range.start + top_unit_range.len())
                    ..(current_include.source_range.start
                        + top_unit_range.len()
                        + bottom_unit_range.len()),
                path: current_include.path,
            },
        );

        // adjust
        for include in &mut self.includes[self.current_include + 2..] {
            include.unit_range.start += file.len();
            include.unit_range.end += file.len();
        }

        self.current_include += 1;

        Ok(())
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
