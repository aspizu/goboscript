use std::{
    fs::File,
    io::{self, BufReader, Read},
    ops::Range,
    path::PathBuf,
};

use fxhash::FxHashMap;

#[derive(Debug)]
pub struct Include {
    pub range: Range<usize>,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Macro {
    pub args: Vec<String>,
    pub substitution: String,
}

#[derive(Default)]
pub struct PreProc {
    pub basepath: PathBuf,
    pub buffer: Vec<u8>,
    pub includes: Vec<Include>,
    pub defines: FxHashMap<String, String>,
    pub macros: FxHashMap<String, Macro>,
}

impl PreProc {
    pub fn new(basepath: PathBuf) -> Self {
        Self {
            basepath,
            ..Default::default()
        }
    }

    pub fn get_translation_unit(&self) -> &str {
        std::str::from_utf8(&self.buffer).unwrap()
    }

    pub fn include_relative(&mut self, path: PathBuf) -> io::Result<()> {
        self.include(self.basepath.join(path))
    }

    pub fn include(&mut self, path: PathBuf) -> io::Result<()> {
        if self.includes.iter().any(|i| i.path == path) {
            return Ok(());
        }
        let length = self.buffer.len();
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut self.buffer)?;
        self.includes.push(Include {
            range: length..self.buffer.len(),
            path,
        });
        Ok(())
    }

    pub fn process(&mut self) -> io::Result<()> {
        let mut includes: Vec<PathBuf> = vec![];
        let mut line_start = true;
        let mut i = 0;
        while i < self.buffer.len() {
            if line_start {
                if self.buffer[i..].starts_with(b"%include ") {
                    self.buffer[i] = b'#';
                    i += b"%include ".len();
                    let path = self.buffer[i..].split(|&c| c == b'\n').next().unwrap();
                    i += path.len();
                    let path = std::str::from_utf8(path).unwrap();
                    let path = PathBuf::from(path);
                    includes.push(path);
                } else if self.buffer[i..].starts_with(b"%define ") {
                    self.buffer[i] = b'#';
                    i += b"%define ".len();
                    let name = self.buffer[i..]
                        .split(|&c| c == b' ' || c == b'(')
                        .next()
                        .unwrap();
                    i += name.len();
                    if self.buffer[i] == b'(' {
                        i += 1;
                        let mut args = vec![];
                        while self.buffer[i] != b')' {
                            if self.buffer[i] == b',' {
                                i += 1;
                            }
                            let arg = self.buffer[i..]
                                .split(|&c| c == b',' || c == b')')
                                .next()
                                .unwrap();
                            i += arg.len();
                            args.push(std::str::from_utf8(arg).unwrap().to_string());
                        }
                        i += 1;
                        let substitution = self.buffer[i..].split(|&c| c == b'\n').next().unwrap();
                        i += substitution.len();
                        let name = std::str::from_utf8(name).unwrap();
                        let substitution = std::str::from_utf8(substitution).unwrap();
                        self.macros.insert(
                            name.to_string(),
                            Macro {
                                args,
                                substitution: substitution.to_string(),
                            },
                        );
                    } else {
                        i += 1;
                        let value = self.buffer[i..].split(|&c| c == b'\n').next().unwrap();
                        i += value.len();
                        let name = std::str::from_utf8(name).unwrap();
                        let value = std::str::from_utf8(value).unwrap();
                        self.defines.insert(name.to_string(), value.to_string());
                    }
                }
            }
            line_start = self.buffer[i] == b'\n';
            i += 1;
        }
        let is_includes_empty = includes.is_empty();
        for include in includes {
            self.include_relative(include)?;
        }
        if !is_includes_empty {
            self.process()?;
        }
        Ok(())
    }

    pub fn translate_position(&self, position: usize) -> (usize, &Include) {
        for include in &self.includes {
            if include.range.contains(&position) {
                return (position - include.range.start, include);
            }
        }
        panic!("invalid position");
    }
}
