#![feature(normalize_lexically)]
pub mod ast;
pub mod blocks;
pub mod codegen;
pub mod config;
pub mod diagnostic;
pub mod fmt;
pub mod frontend;
pub mod lexer;
pub mod misc;
pub mod parser;
pub mod pre_processor;
#[cfg(not(target_arch = "wasm32"))]
pub mod standard_library;
#[cfg(target_arch = "wasm32")]
pub mod standard_library {
    use std::path::Path;

    use semver::Version;

    pub struct StandardLibrary {
        pub version: Version,
        pub path: std::path::PathBuf,
    }

    pub fn standard_library_from_latest(cache_path: &Path) -> anyhow::Result<StandardLibrary> {
        unreachable!()
    }

    pub fn fetch_standard_library(stdlib: &StandardLibrary) -> anyhow::Result<()> {
        unreachable!()
    }

    pub fn new_standard_library(version: Version, cache_path: &Path) -> StandardLibrary {
        StandardLibrary {
            path: cache_path.join(format!("v{}", version)),
            version,
        }
    }
}
pub mod translation_unit;
pub mod vfs;
pub mod visitor;
pub mod wasm;

use std::panic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn deinitialize() {}
