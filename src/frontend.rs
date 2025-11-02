pub mod build;
mod cli;
mod fmt;
mod new;

use std::process::ExitCode;

use clap::{
    CommandFactory,
    Parser,
};
use cli::{
    Cli,
    Command,
};
use colored::Colorize;
use new::NewError;

use crate::{
    config::Config,
    fmt::FmtError,
};

pub fn frontend() -> ExitCode {
    match Cli::parse().command {
        Command::Build { input, output } => match build::build(input, output) {
            Ok(artifact) => {
                artifact.eprint();
                eprintln!();
                if artifact.failure() {
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                }
            }
            Err(err) => {
                eprintln!("{}: {:?}", "error".red().bold(), err);
                ExitCode::FAILURE
            }
        },
        Command::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
            ExitCode::SUCCESS
        }
        Command::New {
            name,
            no_git,
            std,
            bitmap_resolution,
            frame_rate,
            max_clones,
            no_miscellaneous_limits,
            no_sprite_fencing,
            frame_interpolation,
            high_quality_pen,
            stage_width,
            stage_height,
            makefile,
        } => {
            match new::new(
                name,
                no_git,
                makefile,
                Config {
                    pre_build: None,
                    post_build: None,
                    layers: None,
                    std,
                    bitmap_resolution,
                    frame_rate,
                    max_clones,
                    no_miscellaneous_limits: no_miscellaneous_limits.then_some(true),
                    no_sprite_fencing: no_sprite_fencing.then_some(true),
                    frame_interpolation: frame_interpolation.then_some(true),
                    high_quality_pen: high_quality_pen.then_some(true),
                    stage_width,
                    stage_height,
                },
            ) {
                Err(NewError::AnyhowError(err)) => {
                    eprintln!("{}: {:?}", "error".red().bold(), err);
                    ExitCode::FAILURE
                }
                Err(NewError::NewDirNotEmpty {
                    name,
                    is_name_explicit,
                }) => {
                    eprintln!("{}: {} is not empty", "error".red().bold(), name.display());
                    if !is_name_explicit {
                        eprintln!(
                            "{}: pass a name as a positional argument to create a new directory",
                            "hint".blue().bold()
                        );
                    }
                    ExitCode::FAILURE
                }
                Ok(_) => ExitCode::SUCCESS,
            }
        }
        Command::Fmt { input } => match fmt::fmt(input) {
            Ok(_) => ExitCode::SUCCESS,
            Err(FmtError::AnyhowError(err)) => {
                eprintln!("{}: {:?}", "error".red().bold(), err);
                ExitCode::FAILURE
            }
        },
    }
}
