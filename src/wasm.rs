use std::{
    cell::RefCell,
    io::Cursor,
    rc::Rc,
};

use fxhash::FxHashMap;
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
    codegen::sb3::Sb3,
    diagnostic::SpriteDiagnostics,
    frontend::build::build_impl,
    misc::SmolStr,
    standard_library::StandardLibrary,
    vfs::MemFS,
};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Artifact {
    project: Vec<u8>,
    stage_diagnostics: SpriteDiagnostics,
    sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics>,
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
        sb3,
        Some(stdlib),
    )
    .unwrap();
    serde_wasm_bindgen::to_value(&Artifact {
        project: file,
        sprites_diagnostics: artifact.sprites_diagnostics,
        stage_diagnostics: artifact.stage_diagnostics,
    })
    .unwrap()
}
