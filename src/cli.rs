use std::path::PathBuf;

use clap_derive::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "gsc")]
#[command(about="Goboscript compiler", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command()]
    Build {
        #[arg(short, long)]
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
