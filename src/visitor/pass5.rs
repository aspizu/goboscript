use crate::{
    diagnostic::{
        DiagnosticKind,
        SpriteDiagnostics,
    },
    ast::Project,
    misc::SmolStr,
};
use fxhash::FxHashMap;

pub fn visit_project(
    project: &mut Project,
    stage_diagnostics: &mut SpriteDiagnostics,
    sprites_diagnostics: &mut FxHashMap<SmolStr, SpriteDiagnostics>,
) {
    // Check stage structs
    for (name, struct_) in &project.stage.structs {
        if struct_.fields.is_empty() {
            stage_diagnostics.report(
                DiagnosticKind::EmptyStruct(name.clone()),
                &struct_.span,
            );
        }
    }

    // Check sprite structs
    for (sprite_name, sprite) in &project.sprites {
        if let Some(diagnostics) = sprites_diagnostics.get_mut(sprite_name) {
            for (name, struct_) in &sprite.structs {
                if struct_.fields.is_empty() {
                    diagnostics.report(
                        DiagnosticKind::EmptyStruct(name.clone()),
                        &struct_.span,
                    );
                }
            }
        }
    }
}
