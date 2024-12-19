use annotate_snippets::Renderer;
use fxhash::FxHashMap;
use smol_str::SmolStr;

use super::SpriteDiagnostics;
use crate::ast::Project;

pub struct ProjectDiagnostics {
    pub project: Project,
    pub stage_diagnostics: SpriteDiagnostics,
    pub sprites_diagnostics: FxHashMap<SmolStr, SpriteDiagnostics>,
}

impl ProjectDiagnostics {
    pub fn eprint(&self) {
        let renderer = Renderer::styled();
        self.stage_diagnostics.eprint(&renderer, &self.project);
        for sprite_diagnostics in self.sprites_diagnostics.values() {
            sprite_diagnostics.eprint(&renderer, &self.project);
        }
    }
}
