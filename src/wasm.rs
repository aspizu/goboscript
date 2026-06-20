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
    codegen::{
        build::build_impl,
        sb3::Sb3,
    },
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

type FxHashMap<K, V> = Map<K, V>;

type FxHashSet<K> = Set<K>;

type SmolStr = string;

type Value = boolean | number | string;
";

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Build {
    #[serde(with = "base64")]
    file: Vec<u8>,
    artifact: Artifact,
}

#[wasm_bindgen]
pub fn build(fs: MemFS) -> Build {
    let fs = Rc::new(RefCell::new(fs));
    let mut file = Vec::new();
    let sb3 = Sb3::new(Cursor::new(&mut file), fs.clone(), "project".into());
    let stdlib = StandardLibrary {
        path: "stdlib".into(),
        version: Version::new(0, 0,  0),
    };
    let artifact = build_impl(fs, "project".into(), sb3, Some(stdlib)).unwrap();
    Build { file, artifact }
}

#[wasm_bindgen]
pub fn diagnostic_to_string(diagnostic: Diagnostic, sprite: Sprite) -> String {
    diagnostic.kind.to_string(&sprite)
}
