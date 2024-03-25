use std::{panic, process::ExitCode, time::Instant};

use colored::Colorize;

mod ast;
mod blocks;
mod cli;
mod codegen;
mod config;
mod custom_toml_error;
mod diagnostic;
mod frontend;
mod lexer;
mod parser;
mod preproc;
mod visitors;

fn main() -> ExitCode {
    panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n\n{} 💀\nor open an issue at {}",
            "Let's pretend that didn't happen".bold(),
            "https://github.com/aspizu/goboscript/issues".blue()
        );
    }));
    let begin = Instant::now();
    let result = frontend::frontend();
    if let Err(err) = &result {
        eprintln!("{}{} {}", "error".bold().red(), ":".bold(), err.to_string().bold());
    }
    eprintln!("{} in {:?}", "finished".bold().blue(), begin.elapsed());
    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
