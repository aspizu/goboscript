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
    proc_args: &'a mut FxHashMap<SmolStr, Vec<Arg>>,
    func_args: &'a mut FxHashMap<SmolStr, Vec<Arg>>,
    vars: &'a mut FxHashMap<SmolStr, Var>,
    proc_locals: &'a mut FxHashMap<SmolStr, FxHashMap<SmolStr, Var>>,
    func_locals: &'a mut FxHashMap<SmolStr, FxHashMap<SmolStr, Var>>,
    lists: &'a mut FxHashMap<SmolStr, List>,
    structs: &'a mut FxHashMap<SmolStr, Struct>,
    enums: &'a mut FxHashMap<SmolStr, Enum>,
    global_vars: Option<&'a mut FxHashMap<SmolStr, Var>>,
    global_lists: Option<&'a mut FxHashMap<SmolStr, List>>,
}

impl Scope<'_> {
    fn mark_struct_field(
        refr: &NameReference,
        structs: &mut FxHashMap<SmolStr, Struct>,
        vars: &FxHashMap<SmolStr, Var>,
    ) {
        let Some(field) = &refr.field else { return };
        let Some((type_name, _)) = vars[&refr.name].type_.struct_() else {
            return;
        };
        let Some(struct_) = structs.get_mut(type_name) else {
            return;
        };
        let Some(f) = struct_.fields.iter_mut().find(|f| &f.name == field) else {
            return;
        };
        f.is_used = true;
    }

    fn mark_arg_struct_field(
        refr: &NameReference,
        structs: &mut FxHashMap<SmolStr, Struct>,
        args: &[Arg],
    ) {
        let Some(field) = &refr.field else { return };
        let Some((type_name, _)) = args
            .iter()
            .find(|a| a.name == refr.name)
            .and_then(|a| a.type_.struct_())
        else {
            return;
        };
        let Some(struct_) = structs.get_mut(type_name) else {
            return;
        };
        let Some(f) = struct_.fields.iter_mut().find(|f| &f.name == field) else {
            return;
        };
        f.is_used = true;
    }
}

pub fn visit_project(project: &mut Project) {
    // first, visit the stage
    for event in &project.stage.events {
        resolve_references(
            &mut Scope {
                used_procs: &mut project.stage.used_procs,
                used_funcs: &mut project.stage.used_funcs,
                proc_args: &mut project.stage.proc_args,
                func_args: &mut project.stage.func_args,
                vars: &mut project.stage.vars,
                proc_locals: &mut project.stage.proc_locals,
                func_locals: &mut project.stage.func_locals,
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
                    proc_args: &mut sprite.proc_args,
                    func_args: &mut sprite.func_args,
                    vars: &mut sprite.vars,
                    proc_locals: &mut sprite.proc_locals,
                    func_locals: &mut sprite.func_locals,
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
    for refr in &references.args {
        if let Some(arg) = refr
            .proc
            .as_ref()
            .and_then(|p| scope.proc_args.get_mut(p))
            .and_then(|a| a.iter_mut().find(|a| a.name == refr.name))
        {
            arg.is_used = true;
            Scope::mark_arg_struct_field(
                refr,
                scope.structs,
                scope
                    .proc_args
                    .get_mut(refr.proc.as_ref().unwrap())
                    .unwrap(),
            );
            continue;
        }
        if let Some(arg) = refr.func.as_ref().and_then(|p| {
            scope
                .func_args
                .get_mut(p)
                .and_then(|a| a.iter_mut().find(|a| a.name == refr.name))
        }) {
            arg.is_used = true;
            Scope::mark_arg_struct_field(
                refr,
                scope.structs,
                scope
                    .func_args
                    .get_mut(refr.func.as_ref().unwrap())
                    .unwrap(),
            );
            continue;
        }
    }
    for refr in &references.names {
        if let Some(var) = refr
            .proc
            .as_ref()
            .and_then(|p| scope.proc_locals.get_mut(p))
            .and_then(|l| l.get_mut(&refr.name))
        {
            var.is_used = true;
            Scope::mark_struct_field(
                refr,
                scope.structs,
                scope
                    .proc_locals
                    .get_mut(refr.proc.as_ref().unwrap())
                    .unwrap(),
            );
            continue;
        }
        if let Some(var) = refr
            .func
            .as_ref()
            .and_then(|p| scope.func_locals.get_mut(p))
            .and_then(|l| l.get_mut(&refr.name))
        {
            var.is_used = true;
            Scope::mark_struct_field(
                refr,
                scope.structs,
                scope
                    .func_locals
                    .get_mut(refr.func.as_ref().unwrap())
                    .unwrap(),
            );
            continue;
        }
        if let Some(var) = scope.vars.get_mut(&refr.name) {
            var.is_used = true;
            Scope::mark_struct_field(refr, scope.structs, scope.vars);
            continue;
        }
        if let Some(var) = scope
            .global_vars
            .as_mut()
            .and_then(|g| g.get_mut(&refr.name))
        {
            var.is_used = true;
            Scope::mark_struct_field(refr, scope.structs, scope.global_vars.as_mut().unwrap());
            continue;
        }
        if let Some(list) = scope.lists.get_mut(&refr.name) {
            list.is_used = true;
            continue;
        }
        if let Some(list) = scope
            .global_lists
            .as_mut()
            .and_then(|g| g.get_mut(&refr.name))
        {
            list.is_used = true;
            continue;
        }
        if let Some(enum_) = scope.enums.get_mut(&refr.name) {
            if let Some(variant) = refr
                .field
                .as_ref()
                .and_then(|variant| enum_.variants.iter_mut().find(|v| &v.name == variant))
            {
                variant.is_used = true;
                continue;
            }
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
