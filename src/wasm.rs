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
use tsify::{
    Ts,
    Tsify,
};
use wasm_bindgen::{
    prelude::*,
    JsError,
};

use crate::{
    ast::Sprite,
    codegen::build::build_impl,
    diagnostic::{
        Artifact,
        Diagnostic,
    },
    misc::base64,
    standard_library::StandardLibrary,
    vfs::MemFS,
};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = "
export interface Span {
    start: number
    end: number
}

type FxHashMap<K, V> = Map<K, V>
";

#[derive(Tsify, Serialize, Deserialize)]
pub struct Build {
    #[serde(with = "base64")]
    file: Vec<u8>,
    artifact: Artifact,
}

#[wasm_bindgen]
pub fn build(fs: Ts<MemFS>) -> Result<Ts<Build>, JsError> {
    let fs = fs.to_rust()?;
    let fs = Rc::new(RefCell::new(fs));
    let mut file = Vec::new();
    let stdlib = StandardLibrary {
        path: "stdlib".into(),
        version: Version::new(0, 0, 0),
    };
    let artifact = build_impl(fs, "project".into(), Cursor::new(&mut file), Some(stdlib))
        .map_err(|error| JsError::new(&error.to_string()))?;
    Ok(Build { file, artifact }.into_ts()?)
}

#[wasm_bindgen]
pub fn diagnostic_to_string(
    diagnostic: Ts<Diagnostic>,
    sprite: Ts<Sprite>,
) -> Result<String, JsError> {
    let diagnostic = diagnostic.to_rust()?;
    let sprite = sprite.to_rust()?;
    Ok(diagnostic.kind.to_string(&sprite))
}
