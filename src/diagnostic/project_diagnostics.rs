use annotate_snippets::{
    Level,
    Renderer,
};
use fxhash::FxHashMap;

use super::SpriteDiagnostics;
use crate::{
    ast::Project,
    misc::SmolStr,
};

pub struct Artifact {
    pub project: Project,
    pub stage_diagnostics: SpriteDiagnostics,
    pub sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics>,
}

impl Artifact {
    pub fn eprint(&self) {
        let renderer = Renderer::styled();
        self.stage_diagnostics.eprint(&renderer, &self.project);
        for sprite_diagnostics in self.sprites_diagnostics.values() {
            sprite_diagnostics.eprint(&renderer, &self.project);
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
