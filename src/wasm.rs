use std::{
    cell::RefCell,
    io::Cursor,
    rc::Rc,
};

use fxhash::FxHashMap;
use js_sys::Uint8Array;
use semver::Version;
use serde::{
    Deserialize,
    Serialize,
};
use wasm_bindgen::JsValue;

use crate::{
    codegen::sb3::Sb3,
    diagnostic::SpriteDiagnostics,
    frontend::build::{
        build_impl,
        BuildError,
    },
    misc::SmolStr,
    standard_library::StandardLibrary,
    vfs::MemFS,
};

#[derive(Serialize, Deserialize)]
pub struct Diagnostics {
    stage_diagnostics: SpriteDiagnostics,
    sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics>,
}

pub fn build(fs: JsValue) -> JsValue {
    let fs: MemFS = serde_wasm_bindgen::from_value(fs).unwrap();
    let mut file = Vec::new();
    let sb3 = Sb3::new(Cursor::new(&mut file));
    let stdlib = StandardLibrary {
        path: "stdlib".into(),
        version: Version::new(0, 0, 0),
    };
    match build_impl(Rc::new(RefCell::new(fs)), "".into(), sb3, Some(stdlib)) {
        Ok(()) => {
            let uint8_array = Uint8Array::from(file.as_slice());
            uint8_array.into()
        }
        Err(BuildError::ProjectDiagnostics(diagnostics)) => {
            serde_wasm_bindgen::to_value(&Diagnostics {
                stage_diagnostics: diagnostics.stage_diagnostics,
                sprites_diagnostics: diagnostics.sprites_diagnostics,
            })
            .unwrap()
        }
        Err(_) => panic!(),
    }
}
