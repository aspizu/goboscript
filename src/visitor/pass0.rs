use std::path::Path;

use fxhash::FxHashMap;
use glob::glob;

use crate::{
    ast::*,
    codegen::{
        costumes::{
            BITMAP_FORMATS,
            VECTOR_FORMATS,
        },
        sounds::SOUND_FORMATS,
    },
    misc::SmolStr,
};

struct V<'a> {
    locals: Option<&'a mut FxHashMap<SmolStr, Var>>,
    vars: &'a mut FxHashMap<SmolStr, Var>,
    global_vars: Option<&'a mut FxHashMap<SmolStr, Var>>,
}

pub fn visit_project(input: &Path, project: &mut Project) {
    visit_sprite(input, &mut project.stage, None);
    for sprite in project.sprites.values_mut() {
        visit_sprite(input, sprite, Some(&mut project.stage));
    }
}

fn visit_sprite(input: &Path, sprite: &mut Sprite, mut stage: Option<&mut Sprite>) {
    visit_assets(input, &mut sprite.costumes, true);
    visit_assets(input, &mut sprite.sounds, false);
    for enum_ in sprite.enums.values_mut() {
        visit_enum(enum_);
    }
    for proc in sprite.procs.values_mut() {
        sprite
            .proc_locals
            .insert(proc.name.clone(), Default::default());
        let proc_definition = sprite.proc_definitions.get_mut(&proc.name).unwrap();
        visit_stmts(
            proc_definition,
            &mut V {
                locals: sprite.proc_locals.get_mut(&proc.name),
                vars: &mut sprite.vars,
                global_vars: stage.as_mut().map(|stage| &mut stage.vars),
            },
        );
    }
    for func in sprite.funcs.values_mut() {
        sprite
            .func_locals
            .insert(func.name.clone(), Default::default());
        let name: SmolStr = format!("{}:return", func.name).into();
        if !sprite.vars.contains_key(&name) {
            sprite.vars.insert(
                name.clone(),
                Var {
                    name,
                    span: func.span.clone(),
                    type_: func.type_.clone(),
                    default: None,
                    is_cloud: false,
                    is_used: true,
                },
            );
        }
        let func_definition = sprite.func_definitions.get_mut(&func.name).unwrap();
        visit_stmts(
            func_definition,
            &mut V {
                locals: sprite.func_locals.get_mut(&func.name),
                vars: &mut sprite.vars,
                global_vars: stage.as_mut().map(|stage| &mut stage.vars),
            },
        );
    }
    for event in &mut sprite.events {
        visit_stmts(
            &mut event.body,
            &mut V {
                locals: None,
                vars: &mut sprite.vars,
                global_vars: stage.as_mut().map(|stage| &mut stage.vars),
            },
        );
    }
}

fn visit_enum(enum_: &mut Enum) {
    let mut index = 0.0;
    for variant in &mut enum_.variants {
        if let Some((value, _)) = &variant.value {
            if let Value::Number(number) = value {
                index = *number;
            }
        } else {
            variant.value = Some((Value::Number(index), variant.span.clone()));
            index += 1.0;
        }
    }
}

fn visit_assets(input: &Path, assets: &mut Vec<Asset>, allow_ascii: bool) {
    let mut i = 0;
    while i < assets.len() {
        if allow_ascii {
            if let Some(suffix) = assets[i].name.strip_prefix("@ascii/") {
                let asset = assets.remove(i);
                for ch in ' '..'~' {
                    let mut new_asset = asset.clone();
                    new_asset.name = format!("{suffix}{ch}").into();
                    assets.insert(i, new_asset);
                    i += 1;
                }
                continue;
            }
        }
        if assets[i].path.contains('*') {
            let asset = assets.remove(i);
            let mut files: Vec<_> = glob(input.join(asset.path.as_str()).to_str().unwrap())
                .unwrap()
                .flatten()
                .collect();
            files.sort();
            for file in files {
                let Some(ext) = file.extension() else {
                    continue;
                };
                let ext = ext.to_str().unwrap().to_lowercase();
                let ext = ext.as_str();
                if !(BITMAP_FORMATS.contains(&ext)
                    || VECTOR_FORMATS.contains(&ext)
                    || SOUND_FORMATS.contains(&ext))
                {
                    continue;
                }
                let new_asset =
                    Asset::new(file.to_str().unwrap().into(), None, asset.span.clone());
                assets.insert(i, new_asset);
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

fn visit_stmts(stmts: &mut Vec<Stmt>, v: &mut V) {
    for stmt in stmts {
        visit_stmt(stmt, v);
    }
}

fn visit_stmt(stmt: &mut Stmt, v: &mut V) {
    match stmt {
        Stmt::Repeat { body, .. } => visit_stmts(body, v),
        Stmt::Forever { body, .. } => visit_stmts(body, v),
        Stmt::Branch {
            if_body, else_body, ..
        } => {
            visit_stmts(if_body, v);
            visit_stmts(else_body, v)
        }
        Stmt::Until { body, .. } => visit_stmts(body, v),
        Stmt::SetVar {
            name,
            type_,
            is_local,
            is_cloud,
            ..
        } => {
            let basename = name.basename();
            let var = Var {
                name: basename.clone(),
                span: name.span(),
                type_: type_.clone(),
                default: v.vars.get(basename).and_then(|var| var.default.clone()),
                is_cloud: *is_cloud,
                is_used: false,
            };
            if *is_local {
                if let Some(locals) = &mut v.locals {
                    if let Some(existing_declaration) = locals.get(basename) {
                        if existing_declaration.type_.is_value() {
                            locals.insert(basename.clone(), var);
                        }
                    } else {
                        locals.insert(basename.clone(), var);
                    }
                }
                return;
            }
            if v.locals
                .as_ref()
                .is_some_and(|locals| locals.contains_key(basename))
            {
                return;
            }
            if v.global_vars
                .as_ref()
                .is_some_and(|global_vars| global_vars.contains_key(basename))
            {
                return;
            }
            //if let Some(existing_declaration) = v.vars.get(basename) {
            // This condition ensures that variables with a specific type (e.g., a struct type) are not overwritten
            // by a previous statement that didn't specify a type (which defaults to type `Value`).
            // In this context, variables don't need to be explicitly declared if the type is `Value`.
            // The syntax for setting variables is as follows:
            // - For `Value` type: `variable_name = value;`
            // - For a specific struct type: `typeName variable_name = value;`
            //
            // Since the visitor processes every variable assignment statement, this check ensures that if an
            // existing variable has a specific type (not `Value`), it is preserved when a new statement tries to
            // reassign it without a type (defaulting to `Value`). Only variables that are of type `Value` can be
            // overwritten by the new assignment.

            // TODO: Make redeclaration of variables with different struct types an error.
            // if existing_declaration.type_.is_value() {
            //     v.vars.insert(basename.clone(), var);
            // }
            if !v.vars.contains_key(basename)
                && v.global_vars
                    .as_ref()
                    .is_some_and(|vars| !vars.contains_key(basename))
            {
                v.vars.insert(basename.clone(), var);
            }
        }
        _ => (),
    }
}
