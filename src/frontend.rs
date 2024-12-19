mod build;
mod cli;
mod new;

use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use cli::{Cli, Command};
use colored::Colorize;

pub fn frontend() -> ExitCode {
    match Cli::parse().command {
        Command::Build { input, output } => {
            let result = build::build(input, output);
            match result {
                Ok(()) => ExitCode::SUCCESS,
                Err(build::BuildError::AnyhowError(err)) => {
                    eprintln!("{}: {:?}", "error".red().bold(), err);
                    ExitCode::FAILURE
                }
                Err(build::BuildError::ProjectDiagnostics(diagnostics)) => {
                    diagnostics.eprint();
                    eprintln!();
                    ExitCode::FAILURE
                }
            }
        }
        Command::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
            ExitCode::SUCCESS
        }
        _ => panic!(),
        // Command::New {
        //     name,
        //     frame_rate,
        //     max_clones,
        //     no_miscellaneous_limits,
        //     no_sprite_fencing,
        //     frame_interpolation,
        //     high_quality_pen,
        //     stage_width,
        //     stage_height,
        // } => new::new(
        //     name,
        //     Config {
        //         frame_rate,
        //         max_clones,
        //         no_miscellaneous_limits: Some(no_miscellaneous_limits),
        //         no_sprite_fencing: Some(no_sprite_fencing),
        //         frame_interpolation: Some(frame_interpolation),
        //         high_quality_pen: Some(high_quality_pen),
        //         stage_width,
        //         stage_height,
        //     },
        // ),
    }
}
