use std::{
    fmt::{
        self,
        Display,
    },
    str,
};

use fxhash::FxHashMap;
use regex::Regex;
use serde::Serialize;

use crate::misc::SmolStr;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum SymbolKind {
    Proc,
    Func,
    Struct,
    Enum,
    List,
    Define,
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Proc => write!(f, "proc"),
            Self::Func => write!(f, "func"),
            Self::Struct => write!(f, "struct"),
            Self::Enum => write!(f, "enum"),
            Self::List => write!(f, "list"),
            Self::Define => write!(f, "define"),
        }
    }
}

impl SymbolKind {
    pub fn to_symbol(&self, name: &SmolStr) -> Symbol {
        Symbol {
            kind: *self,
            name: name.clone(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: SmolStr,
}

// "kind/name"
impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&format!("{}/{}", self.kind, self.name))
    }
}

pub fn find_docstrings(text: &str) -> FxHashMap<Symbol, SmolStr> {
    let mut symbols: FxHashMap<Symbol, SmolStr> = Default::default();
    let text = text.as_bytes();
    let mut i = 0;
    let mut at_line_start = true;
    let mut comment_block = String::new();
    let re = Regex::new(r"(nowarp\s+)?(?<type>proc|func|struct|enum|list|%define)\s+(?<name>\w+)")
        .unwrap();
    while i < text.len() {
        if at_line_start {
            at_line_start = false;
            while i < text.len() && (text[i] == b' ' || text[i] == b'\t') {
                i += 1;
            }
            if text.get(i) == Some(&b'#') {
                if !comment_block.is_empty() {
                    comment_block.push('\n');
                }
                let begin = i;
                i += 1;
                while i < text.len() && text[i] != b'\n' && text[i] != b'\r' {
                    i += 1;
                }
                comment_block.push_str(str::from_utf8(&text[begin..i]).unwrap());
                if i < text.len() && text[i] == b'\r' {
                    i += 1;
                }
                if i < text.len() && text[i] == b'\n' {
                    i += 1;
                    at_line_start = true;
                }
            } else if !comment_block.is_empty() {
                let begin = i;
                while i < text.len() && text[i] != b'\n' && text[i] != b'\r' {
                    i += 1;
                }
                let following = str::from_utf8(&text[begin..i]).unwrap();
                if i < text.len() && text[i] == b'\r' {
                    i += 1;
                }
                if i < text.len() && text[i] == b'\n' {
                    i += 1;
                    at_line_start = true;
                }
                if let Some(captures) = re.captures(following) {
                    let type_ = captures.name("type").unwrap().as_str();
                    let name: SmolStr = captures.name("name").unwrap().as_str().into();
                    symbols.insert(
                        Symbol {
                            kind: match type_ {
                                "proc" => SymbolKind::Proc,
                                "func" => SymbolKind::Func,
                                "struct" => SymbolKind::Struct,
                                "enum" => SymbolKind::Enum,
                                "list" => SymbolKind::List,
                                "%define" => SymbolKind::Define,
                                _ => unreachable!(),
                            },
                            name,
                        },
                        comment_block.as_str().into(),
                    );
                }
                comment_block.clear();
            }
        } else if text[i] == b'\n' {
            i += 1;
            at_line_start = true;
        } else {
            i += 1;
        }
    }
    symbols
}
