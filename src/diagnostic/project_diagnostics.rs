use annotate_snippets::{
    Level,
    Renderer,
};
use fxhash::FxHashMap;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use super::SpriteDiagnostics;
use crate::{
    ast::Project,
    misc::SmolStr,
};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Artifact {
    pub project: Project,
    pub stage_diagnostics: SpriteDiagnostics,
    pub sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics>,
}

impl Artifact {
    pub fn eprint(&self) {
        let cwd = std::env::current_dir().unwrap().canonicalize().unwrap();
        let renderer = Renderer::styled();
        self.stage_diagnostics
            .eprint(&cwd, &renderer, &self.project);
        for sprite_diagnostics in self.sprites_diagnostics.values() {
            sprite_diagnostics.eprint(&cwd, &renderer, &self.project);
        }
    }

    pub fn failure(&self) -> bool {
        self.stage_diagnostics
            .diagnostics
            .iter()
            .any(|diag| matches!(Level::from(&diag.kind), Level::Error))
            || self.sprites_diagnostics.values().any(|sprite_diagnostics| {
                sprite_diagnostics
                    .diagnostics
                    .iter()
                    .any(|diag| matches!(Level::from(&diag.kind), Level::Error))
            })
    }
}
