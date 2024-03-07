mod ast;
mod blockid;
mod build;
mod cli;
mod codegen;
mod details;
mod lexer;
mod logoslalrpop;
mod reporting;
mod visitors;
mod zipfile;

use std::io;

use build::build;
use clap::Parser;
use cli::{Cli, Commands};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

fn main() -> io::Result<()> {
    match Cli::parse().command {
        Commands::Build { input, output } => build(input, output)?,
    }
    Ok(())
}
