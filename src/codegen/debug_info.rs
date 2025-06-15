use fxhash::FxHashMap;
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use crate::{
    ast::Project,
    diagnostic::SpriteDiagnostics,
    misc::SmolStr,
};

#[derive(Tsify, Serialize, Deserialize, Default)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DebugInfo {
    pub blocks: FxHashMap<String, Span>,
    pub variables: FxHashMap<String, Span>,
    pub lists: FxHashMap<String, Span>,
    pub procs: FxHashMap<String, Span>,
    pub funcs: FxHashMap<String, Span>,
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ArtifactRef<'a> {
    pub project: &'a Project,
    pub stage_diagnostics: &'a SpriteDiagnostics,
    pub sprites_diagnostics: &'a FxHashMap<SmolStr, SpriteDiagnostics>,
}
