mod ast;
mod blocks;
mod codegen;
mod config;
mod diagnostic;
mod fmt;
mod frontend;
mod lexer;
mod misc;
mod parser;
mod pre_processor;
mod standard_library;
mod translation_unit;
mod visitor;

use std::{
    process::ExitCode,
    time::Instant,
};

use colored::Colorize;

fn main() -> ExitCode {
    // unsafe { color_debug::enable() };
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
