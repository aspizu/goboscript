use std::{
    process::ExitCode,
    time::Instant,
};

use colored::{
    Color,
    Colorize,
};
use libgoboscript::frontend::frontend;

fn main() -> ExitCode {
    pretty_env_logger::init();
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n{}\nopen an issue at {}",
            "goboscript is cooked 💀".red().bold(),
            "https://github.com/aspizu/goboscript/issues".cyan()
        );
    }));
    let begin = Instant::now();
    let result = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(frontend)
        .expect("failed to spawn compiler thread")
        .join()
        .unwrap_or(ExitCode::FAILURE);
    let color = if matches!(result, ExitCode::SUCCESS) {
        Color::Green
    } else {
        Color::Red
    };
    eprintln!(
        "{} in {:?}",
        "Finished".color(color).bold(),
        begin.elapsed()
    );
    result
}
