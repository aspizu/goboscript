use fxhash::FxHashMap;
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Default)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DebugInfo {
    pub blocks: FxHashMap<String, Span>,
    pub variables: FxHashMap<String, Span>,
    pub lists: FxHashMap<String, Span>,
    pub procs: FxHashMap<String, Span>,
    pub funcs: FxHashMap<String, Span>,
}
