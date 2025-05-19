use std::{
    fs::OpenOptions,
    io::{
        Read,
        Write,
    },
};

use logos::Span;

use super::{
    value::Value,
    ExceptionResult,
    Interpreter,
};
use crate::{
    ast::Expr,
    interpreter::block::arguments,
    misc::SmolStr,
    throw,
};

/// Returns true if the foreign function exists, false otherwise.
pub fn foreign_proc(
    vm: &mut Interpreter,
    name: &str,
    span: &Span,
    args: &[(Option<(SmolStr, Span)>, Expr)],
) -> ExceptionResult<bool> {
    let mut arg_values = vec![];
    for (_arg_name, arg_expr) in args {
        let arg_value = vm.run_expr(arg_expr)?;
        arg_values.push(arg_value);
    }
    match name {
        "write" => {
            let [id, data] = arguments(arg_values, span)?;
            let id = id.to_number() as usize;
            let data = data.to_string();
            if let Some(file) = vm.files.get_mut(id) {
                match file.write_all(data.as_bytes()) {
                    Ok(()) => {}
                    Err(error) => {
                        vm.vars.insert("errstr".into(), error.to_string().into());
                    }
                }
            }
            Ok(true)
        }
        "close" => {
            let [id] = arguments(arg_values, span)?;
            let id = id.to_number() as usize;
            if let Some(file) = vm.files.get_mut(id) {
                match file.flush() {
                    Ok(()) => {
                        vm.files.remove(id);
                    }
                    Err(error) => {
                        vm.vars.insert("errstr".into(), error.to_string().into());
                    }
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Returns true if the foreign function exists, false otherwise.
pub fn foreign_func(
    vm: &mut Interpreter,
    name: &str,
    span: &Span,
    args: &[(Option<(SmolStr, Span)>, Expr)],
) -> ExceptionResult<Value> {
    let mut arg_values = vec![];
    for (_arg_name, arg_expr) in args {
        let arg_value = vm.run_expr(arg_expr)?;
        arg_values.push(arg_value);
    }
    match name {
        "open" => {
            let [filename, mode] = arguments(arg_values, span)?;
            let filename = filename.to_string();
            let mode = mode.to_string();
            match OpenOptions::new()
                .read(mode.contains('r'))
                .write(mode.contains('w'))
                .append(mode.contains('a'))
                .create(mode.contains('w'))
                .truncate(mode.contains('t'))
                .open(filename.as_str())
            {
                Ok(file) => {
                    let id = vm.files.len() as f64;
                    vm.files.push(file);
                    Ok(id.into())
                }
                Err(error) => {
                    vm.vars.insert("errstr".into(), error.to_string().into());
                    Ok(0.0.into())
                }
            }
        }
        "read" => {
            let [id, size] = arguments(arg_values, span)?;
            let id = id.to_number() as usize;
            let size = size.to_number();
            if id >= vm.files.len() {
                return Ok("".into());
            }
            let file = &mut vm.files[id];
            if size == -1.0 {
                let mut buf = String::new();
                let _ = file.read_to_string(&mut buf).map_err(|err| {
                    vm.vars.insert("errstr".into(), err.to_string().into());
                    err
                });
                return Ok(buf.into());
            }
            let mut buffer = vec![0; size as usize];
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    let result = String::from_utf8_lossy(&buffer).to_string();
                    Ok(result.into())
                }
                Err(error) => {
                    vm.vars.insert("errstr".into(), error.to_string().into());
                    Ok("".into())
                }
            }
        }
        _ => {
            throw!(
                format!("{name} is not a valid foreign function"),
                span.clone()
            );
        }
    }
}
