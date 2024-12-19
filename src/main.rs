mod ast;
mod blocks;
mod codegen;
mod config;
mod diagnostic;
mod frontend;
mod lexer;
mod misc;
mod parser;
mod visitor;
use std::{process::ExitCode, time::Instant};

use colored::Colorize;

fn main() -> ExitCode {
    pretty_env_logger::init();
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n{}\nopen an issue at {}",
            "-9999 aura 💀".red().bold(),
            "https://github.com/aspizu/goboscript/issues".cyan()
        );
    }));
    let begin = Instant::now();
    let result = frontend::frontend();
    eprintln!("{} in {:?}", "Finished".green().bold(), begin.elapsed());
    result
}
