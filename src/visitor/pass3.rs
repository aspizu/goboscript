use fxhash::{
    FxHashMap,
    FxHashSet,
};

use crate::{
    ast::*,
    misc::SmolStr,
};

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

impl Scope<'_> {
    fn mark_struct_field(&mut self, name: &str, field: &Option<SmolStr>) {
        let Some(field) = field else { return };
        let Some((type_name, _)) = self.vars[name].type_.struct_() else {
            return;
        };
        let Some(struct_) = self.structs.get_mut(type_name) else {
            return;
        };
        let Some(f) = struct_.fields.iter_mut().find(|f| &f.name == field) else {
            return;
        };
        f.is_used = true;
    }

    fn mark_enum_variant(&mut self, name: &str, field: &Option<SmolStr>) {
        let Some(field) = field else { return };
        let Some(enum_) = self.enums.get_mut(name) else {
            return;
        };
        let Some(variant) = enum_.variants.iter_mut().find(|v| &v.name == field) else {
            return;
        };
        variant.is_used = true;
    }
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
            &project.stage.proc_references,
            &project.stage.funcs,
            &project.stage.func_references,
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
                &sprite.proc_references,
                &sprite.funcs,
                &sprite.func_references,
                &event.references,
            );
        }
    }
}

fn resolve_references(
    scope: &mut Scope,
    procs: &FxHashMap<SmolStr, Proc>,
    proc_references: &FxHashMap<SmolStr, References>,
    funcs: &FxHashMap<SmolStr, Func>,
    func_references: &FxHashMap<SmolStr, References>,
    references: &References,
) {
    for (name, field) in &references.names {
        if let Some(var) = scope
            .global_vars
            .as_mut()
            .and_then(|global_vars| global_vars.get_mut(name))
        {
            var.is_used = true;
            scope.mark_struct_field(&name, field);
            continue;
        }
        if let Some(var) = scope.vars.get_mut(name) {
            var.is_used = true;
            scope.mark_struct_field(&name, field);
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
        if let Some(enum_) = scope.enums.get_mut(name) {
            enum_.is_used = true;
            scope.mark_enum_variant(name, field);
        }
    }
    for struct_name in &references.structs {
        if let Some(struct_) = scope.structs.get_mut(struct_name) {
            struct_.is_used = true;
        }
    }
    for proc in &references.procs {
        if scope.used_procs.insert(proc.clone()) {
            if let Some(proc) = procs.get(proc) {
                resolve_references(
                    scope,
                    procs,
                    proc_references,
                    funcs,
                    func_references,
                    proc_references.get(&proc.name).unwrap(),
                );
            }
        }
    }
    for func in &references.funcs {
        if scope.used_funcs.insert(func.clone()) {
            if let Some(func) = funcs.get(func) {
                resolve_references(
                    scope,
                    procs,
                    proc_references,
                    funcs,
                    func_references,
                    func_references.get(&func.name).unwrap(),
                );
            }
        }
    }
}
