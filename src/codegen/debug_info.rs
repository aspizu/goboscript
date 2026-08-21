use rustc_hash::FxHashMap;
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize, Default)]
pub struct DebugInfo {
    pub blocks: FxHashMap<String, Span>,
    pub variables: FxHashMap<String, Span>,
    pub lists: FxHashMap<String, Span>,
    pub procs: FxHashMap<String, Span>,
    pub funcs: FxHashMap<String, Span>,
}
