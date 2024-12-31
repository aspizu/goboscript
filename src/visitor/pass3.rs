use fxhash::{FxHashMap, FxHashSet};
use smol_str::SmolStr;

use crate::ast::*;

struct Scope<'a> {
    used_procs: &'a mut FxHashSet<SmolStr>,
    used_funcs: &'a mut FxHashSet<SmolStr>,
    vars: &'a mut FxHashMap<SmolStr, Var>,
    lists: &'a mut FxHashMap<SmolStr, List>,
    structs: &'a mut FxHashMap<SmolStr, Struct>,
    enums: &'a mut FxHashMap<SmolStr, Enum>,
    global_vars: Option<&'a mut FxHashMap<SmolStr, Var>>,
    global_lists: Option<&'a mut FxHashMap<SmolStr, List>>,
}

pub fn visit_project(project: &mut Project) {
    // first, visit the stage
    for event in &project.stage.events {
        resolve_references(
            &mut Scope {
                used_procs: &mut project.stage.used_procs,
                used_funcs: &mut project.stage.used_funcs,
                vars: &mut project.stage.vars,
                lists: &mut project.stage.lists,
                structs: &mut project.stage.structs,
                enums: &mut project.stage.enums,
                global_vars: None,
                global_lists: None,
            },
            &project.stage.procs,
            &project.stage.funcs,
            &event.references,
        );
    }

    // then visit each sprite
    for sprite in project.sprites.values_mut() {
        for event in &sprite.events {
            resolve_references(
                &mut Scope {
                    used_procs: &mut sprite.used_procs,
                    used_funcs: &mut sprite.used_funcs,
                    vars: &mut sprite.vars,
                    lists: &mut sprite.lists,
                    structs: &mut sprite.structs,
                    enums: &mut sprite.enums,
                    global_vars: Some(&mut project.stage.vars),
                    global_lists: Some(&mut project.stage.lists),
                },
                &sprite.procs,
                &sprite.funcs,
                &event.references,
            );
        }
    }
}

fn resolve_references(
    scope: &mut Scope,
    procs: &FxHashMap<SmolStr, Proc>,
    funcs: &FxHashMap<SmolStr, Func>,
    references: &References,
) {
    for name in &references.names {
        if let Some(global_vars) = &mut scope.global_vars {
            if let Some(var) = global_vars.get_mut(name) {
                var.is_used = true;
                continue;
            }
        }
        if let Some(var) = scope.vars.get_mut(name) {
            var.is_used = true;
        }
        if let Some(global_lists) = &mut scope.global_lists {
            if let Some(list) = global_lists.get_mut(name) {
                list.is_used = true;
                continue;
            }
        }
        if let Some(list) = scope.lists.get_mut(name) {
            list.is_used = true;
        }
    }
    for struct_name in &references.structs {
        if let Some(struct_) = scope.structs.get_mut(struct_name) {
            struct_.is_used = true;
        }
    }
    for (struct_name, field_name) in &references.struct_fields {
        if let Some(struct_) = &mut scope.structs.get_mut(struct_name) {
            let struct_field = &mut struct_
                .fields
                .iter_mut()
                .find(|field| &field.name == field_name)
                .unwrap();
            struct_field.is_used = true;
        }
    }
    for (enum_name, variant_name) in &references.enum_variants {
        if let Some(enum_) = &mut scope.enums.get_mut(enum_name) {
            let enum_variant = &mut enum_
                .variants
                .iter_mut()
                .find(|variant| &variant.name == variant_name)
                .unwrap();
            enum_variant.is_used = true;
        }
    }
    for proc in &references.procs {
        if scope.used_procs.insert(proc.clone()) {
            if let Some(proc) = procs.get(proc) {
                resolve_references(scope, procs, funcs, &proc.references);
            }
        }
    }
    for func in &references.funcs {
        if scope.used_funcs.insert(func.clone()) {
            if let Some(proc) = funcs.get(func) {
                resolve_references(scope, procs, funcs, &proc.references);
            }
        }
    }
}
