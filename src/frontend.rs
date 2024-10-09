pub mod build;
pub mod new;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    cli::{Cli, Commands},
    config::Config,
};

pub fn frontend() -> Result<()> {
    match Cli::parse().command {
        Commands::Build { input, output, compact } => {
            build::build(input, output, compact)
        }
        Commands::New {
            name,
            frame_rate,
            max_clones,
            no_miscellaneous_limits,
            no_sprite_fencing,
            frame_interpolation,
            high_quality_pen,
            stage_width,
            stage_height,
        } => new::new(
            name,
            Config {
                frame_rate,
                max_clones,
                no_miscellaneous_limits: no_miscellaneous_limits.then_some(true),
                no_sprite_fencing: no_sprite_fencing.then_some(true),
                frame_interpolation: frame_interpolation.then_some(true),
                high_quality_pen: high_quality_pen.then_some(true),
                stage_width,
                stage_height,
            },
        ),
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
            Ok(())
        }
    }
}
