pub mod build;
mod cli;
mod fmt;
mod new;
mod run;

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
use fmt::FmtError;
use new::NewError;
use run::RunError;

use crate::config::Config;

pub fn frontend() -> ExitCode {
    match Cli::parse().command {
        Command::Build { input, output } => match build::build(input, output) {
            Ok(()) => ExitCode::SUCCESS,
            Err(build::BuildError::AnyhowError(err)) => {
                eprintln!("{}: {:?}", "error".red().bold(), err);
                ExitCode::FAILURE
            }
            Err(build::BuildError::ProjectDiagnostics(diagnostics)) => {
                diagnostics.eprint();
                eprintln!();
                if diagnostics.failure() {
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                }
            }
        },
        Command::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
            ExitCode::SUCCESS
        }
        Command::New {
            name,
            std,
            frame_rate,
            max_clones,
            no_miscellaneous_limits,
            no_sprite_fencing,
            frame_interpolation,
            high_quality_pen,
            stage_width,
            stage_height,
        } => {
            match new::new(
                name,
                Config {
                    std,
                    frame_rate,
                    max_clones,
                    no_miscellaneous_limits: Some(no_miscellaneous_limits),
                    no_sprite_fencing: Some(no_sprite_fencing),
                    frame_interpolation: Some(frame_interpolation),
                    high_quality_pen: Some(high_quality_pen),
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
                        eprintln!("{}: use --name to specify a name", "hint".blue().bold());
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
        Command::Run { input } => match run::run(input) {
            Ok(_) => ExitCode::SUCCESS,
            Err(RunError::ProjectDiagnostics(diagnostics)) => {
                diagnostics.eprint();
                eprintln!();
                if diagnostics.failure() {
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                }
            }
            Err(RunError::AnyhowError(err)) => {
                eprintln!("{}: {:?}", "error".red().bold(), err);
                ExitCode::FAILURE
            }
        },
    }
}
