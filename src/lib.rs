#![feature(normalize_lexically)]

pub mod ast;
pub mod blocks;
pub mod codegen;
pub mod config;
pub mod diagnostic;
pub mod lexer;
pub mod misc;
pub mod parser;
pub mod pre_processor;

#[cfg(not(target_arch = "wasm32"))]
pub mod frontend;

#[cfg(not(target_arch = "wasm32"))]
pub mod fmt;

#[cfg(not(target_arch = "wasm32"))]
pub mod standard_library;

#[cfg(target_arch = "wasm32")]
pub mod standard_library {
    use std::path::Path;

    use semver::Version;

    #[derive(Debug)]
    pub struct StandardLibrary {
        pub version: Version,
        pub path: std::path::PathBuf,
    }

    pub fn standard_library_from_latest(cache_path: &Path) -> anyhow::Result<StandardLibrary> {
        unreachable!("standard_library_from_latest is not implemented for wasm")
    }

    pub fn fetch_standard_library(stdlib: &StandardLibrary) -> anyhow::Result<()> {
        unreachable!("fetch_standard_library is not implemented for wasm")
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
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Warn).unwrap();
}

#[wasm_bindgen]
pub fn deinitialize() {}
