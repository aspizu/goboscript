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
pub mod standard_library;
pub mod translation_unit;
pub mod vfs;
pub mod visitor;

use std::panic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn deinitialize() {}
