use std::path::Path;

use fxhash::FxHashMap;
use glob::glob;

use crate::{
    ast::*,
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
    visit_costumes(input, &mut sprite.costumes);
    visit_sounds(input, &mut sprite.sounds);
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

fn visit_costumes(input: &Path, new: &mut Vec<Costume>) {
    let old: Vec<Costume> = std::mem::take(new);
    for costume in old {
        if let Some(suffix) = costume.name.strip_prefix("@ascii/") {
            new.extend((' '..='~').map(|ch| Costume {
                name: format!("{suffix}{ch}").into(),
                path: costume.path.clone(),
                span: costume.span.clone(),
            }));
        } else if costume.path.contains('*') {
            let mut costumes: Vec<Costume> = glob(input.join(&*costume.path).to_str().unwrap())
                .unwrap()
                .map(Result::unwrap)
                .map(|path| Costume {
                    name: path.file_stem().unwrap().to_string_lossy().into(),
                    path: path.to_string_lossy().into(),
                    span: costume.span.clone(),
                })
                .collect();
            costumes.sort_by(|a, b| a.name.cmp(&b.name));
            new.extend(costumes);
        } else {
            new.push(costume);
        }
    }
}

fn visit_sounds(input: &Path, new: &mut Vec<Sound>) {
    let old: Vec<Sound> = std::mem::take(new);
    for sound in old {
        if let Some(suffix) = sound.name.strip_prefix("@ascii/") {
            new.extend((' '..='~').map(|ch| Sound {
                name: format!("{suffix}{ch}").into(),
                path: sound.path.clone(),
                span: sound.span.clone(),
            }));
        } else if sound.path.contains('*') {
            let mut sounds: Vec<Sound> = glob(input.join(&*sound.path).to_str().unwrap())
                .unwrap()
                .map(Result::unwrap)
                .map(|path| Sound {
                    name: path.file_stem().unwrap().to_string_lossy().into(),
                    path: path.to_string_lossy().into(),
                    span: sound.span.clone(),
                })
                .collect();
            sounds.sort_by(|a, b| a.name.cmp(&b.name));
            new.extend(sounds);
        } else {
            new.push(sound);
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
