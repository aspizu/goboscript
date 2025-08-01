use std::path::PathBuf;

use clap_derive::{
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile a goboscript project to `.sb3`
    #[command(alias = "b")]
    Build {
        #[arg()]
        /// Project directory, if not given, the current directory is used.
        input: Option<PathBuf>,
        #[arg(short, long)]
        /// Output file, if not given, it will be the project directory's name + `.sb3`
        output: Option<PathBuf>,
    },

    /// Create a new goboscript project with a blank backdrop, a main sprite with a
    /// blank costume.
    #[command(alias = "n")]
    New {
        /// Name of directory to create new project, if not given, the current directory
        /// is used. If this is a path to an existing directory, it must be empty.
        #[arg()]
        name: Option<PathBuf>,

        /// Do not initialize a Git repository.
        #[arg(short = 'G', long)]
        no_git: bool,

        /// Version of the standard library to use. Defaults to bleeding-edge.
        #[arg(short = 's', long)]
        std: Option<String>,

        // (alias: --bmpres) Bitmap resolution. (default: 1)
        #[arg(short = 'b', long, alias = "bmpres")]
        bitmap_resolution: Option<u64>,

        /// (alias: --fps) Custom frame rate, used by TurboWarp.
        #[arg(short = 'f', long, alias = "fps")]
        frame_rate: Option<u64>,

        /// (alias: --clones) Custom maximum number of clones allowed, used by TurboWarp.
        /// Use `--max-clones inf` for infinite clones.
        #[arg(short = 'c', long, alias = "clones")]
        max_clones: Option<f64>,

        /// (alias: --limitless) Disable miscellaneous limits, used by TurboWarp.
        #[arg(short = 'l', long, alias = "limitless")]
        no_miscellaneous_limits: bool,

        /// (alias: --offscreen) Disable sprite fencing, used by TurboWarp.
        #[arg(short = 'o', long, alias = "offscreen")]
        no_sprite_fencing: bool,

        /// (alias: --interpolate) Enable frame interpolation, used by TurboWarp.
        #[arg(short = 'i', long, alias = "interpolate")]
        frame_interpolation: bool,

        /// (alias: --hqpen) Enable high quality pen, used by TurboWarp.
        #[arg(short = 'q', long, alias = "hqpen")]
        high_quality_pen: bool,

        /// (alias: --width) Custom stage width, used by TurboWarp.
        #[arg(short = 'W', long, alias = "width")]
        stage_width: Option<u64>,

        /// (alias: --height) Custom stage height, used by TurboWarp.
        #[arg(short = 'H', long, alias = "height")]
        stage_height: Option<u64>,
    },

    /// Format a goboscript project.
    #[command(alias = "f")]
    Fmt {
        /// Project directory or file, if not given, the current directory is used.
        #[arg(short, long)]
        input: Option<PathBuf>,
    },

    /// Generate completions for a shell.
    #[command()]
    Completions {
        /// The shell to generate the completions for.
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
