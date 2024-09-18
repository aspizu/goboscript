use fxhash::{FxHashMap, FxHashSet};
use smol_str::SmolStr;

use crate::ast::{Enum, List, Proc, Project, References, Var};

struct Scope<'a> {
    used_procs: &'a mut FxHashSet<SmolStr>,
    vars: &'a mut FxHashMap<SmolStr, Var>,
    lists: &'a mut FxHashMap<SmolStr, List>,
    enums: &'a mut FxHashMap<SmolStr, Enum>,
    global_broadcasts: &'a mut FxHashSet<SmolStr>,
    global_vars: Option<&'a mut FxHashMap<SmolStr, Var>>,
    global_lists: Option<&'a mut FxHashMap<SmolStr, List>>,
}

pub fn visit_project(project: &mut Project) {
    // first, visit the stage
    for event in &project.stage.events {
        resolve_references(
            &mut Scope {
                used_procs: &mut project.stage.used_procs,
                vars: &mut project.stage.vars,
                lists: &mut project.stage.lists,
                enums: &mut project.stage.enums,
                global_broadcasts: &mut project.stage.broadcasts,
                global_vars: None,
                global_lists: None,
            },
            &project.stage.procs,
            &event.references,
        );
    }

    for on_message in &project.stage.on_messages {
        resolve_references(
            &mut Scope {
                used_procs: &mut project.stage.used_procs,
                vars: &mut project.stage.vars,
                lists: &mut project.stage.lists,
                enums: &mut project.stage.enums,
                global_broadcasts: &mut project.stage.broadcasts,
                global_vars: None,
                global_lists: None,
            },
            &project.stage.procs,
            &on_message.1.references,
        );
    }

    // then visit each sprite
    for sprite in project.sprites.values_mut() {
        for event in &sprite.events {
            resolve_references(
                &mut Scope {
                    used_procs: &mut sprite.used_procs,
                    vars: &mut sprite.vars,
                    lists: &mut sprite.lists,
                    enums: &mut sprite.enums,
                    global_broadcasts: &mut project.stage.broadcasts,
                    global_vars: Some(&mut project.stage.vars),
                    global_lists: Some(&mut project.stage.lists),
                },
                &sprite.procs,
                &event.references,
            );
        }

        for on_message in &sprite.on_messages {
            resolve_references(
                &mut Scope {
                    used_procs: &mut sprite.used_procs,
                    vars: &mut sprite.vars,
                    lists: &mut sprite.lists,
                    enums: &mut sprite.enums,
                    global_broadcasts: &mut project.stage.broadcasts,
                    global_vars: Some(&mut project.stage.vars),
                    global_lists: Some(&mut project.stage.lists),
                },
                &sprite.procs,
                &on_message.1.references,
            );
        }
    }

    // make sure that at least one broadcast exists
    if project.stage.broadcasts.is_empty() {
        project.stage.broadcasts.insert("message1".into());
    }
}

fn resolve_references(
    scope: &mut Scope,
    procs: &FxHashMap<SmolStr, Proc>,
    references: &References,
) {
    for var in &references.vars {
        if let Some(global_vars) = &mut scope.global_vars {
            if let Some(var) = global_vars.get_mut(var) {
                var.used = true;
                continue;
            }
        }
        if let Some(var) = scope.vars.get_mut(var) {
            var.used = true;
        }
    }
    for list in &references.lists {
        if let Some(global_lists) = &mut scope.global_lists {
            if let Some(list) = global_lists.get_mut(list) {
                list.used = true;
                continue;
            }
        }
        if let Some(list) = scope.lists.get_mut(list) {
            list.used = true;
        }
    }
    for (enum_name, variant_name) in &references.enum_variants {
        let enum_ = &mut scope.enums.get_mut(enum_name).unwrap();
        enum_.used_variants.insert(variant_name.clone());
    }
    for message in &references.messages {
        scope.global_broadcasts.insert(message.clone());
    }
    for proc in &references.procs {
        if scope.used_procs.insert(proc.clone()) {
            if let Some(proc) = procs.get(proc) {
                resolve_references(scope, procs, &proc.references);
            }
        }
    }
}
