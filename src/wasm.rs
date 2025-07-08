use std::{
    cell::RefCell,
    io::Cursor,
    rc::Rc,
};

use semver::Version;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;
use wasm_bindgen::{
    prelude::*,
    JsValue,
};

use crate::{
    ast::Sprite,
    codegen::sb3::Sb3,
    diagnostic::{
        Artifact,
        Diagnostic,
    },
    frontend::build::build_impl,
    misc::base64,
    standard_library::StandardLibrary,
    vfs::MemFS,
};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Build {
    #[serde(with = "base64")]
    file: Vec<u8>,
    artifact: Artifact,
}

#[wasm_bindgen]
pub fn build(fs: JsValue) -> JsValue {
    let fs: MemFS = serde_wasm_bindgen::from_value(fs).unwrap();
    let mut file = Vec::new();
    let sb3 = Sb3::new(Cursor::new(&mut file));
    let stdlib = StandardLibrary {
        path: "stdlib".into(),
        version: Version::new(0, 0, 0),
    };
    let artifact = build_impl(
        Rc::new(RefCell::new(fs)),
        "project".into(),
        "project".into(),
        sb3,
        Some(stdlib),
    )
    .unwrap();
    serde_wasm_bindgen::to_value(&Build { file, artifact }).unwrap()
}

#[wasm_bindgen]
pub fn diagnostic_to_string(diagnostic: JsValue, sprite: JsValue) -> String {
    let diagnostic: Diagnostic = serde_wasm_bindgen::from_value(diagnostic).unwrap();
    let sprite: Sprite = serde_wasm_bindgen::from_value(sprite).unwrap();
    diagnostic.kind.to_string(&sprite)
}
