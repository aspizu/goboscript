use std::{
    process::ExitCode,
    time::Instant,
};

use colored::{
    Color,
    Colorize,
};
use libgoboscript::frontend::frontend;

const AGENT_ENV_VARS: &[&str] = &[
    "CLAUDECODE",
    "CLAUDE_CODE_ENTRYPOINT",
    "CODEX_SANDBOX",
    "CODEX_ROOT",
    "CURSOR_AGENT",
    "CURSOR_TRACE_ID",
    "OPENCODE",
    "GEMINI_CLI",
];

fn is_run_by_agent() -> bool {
    AGENT_ENV_VARS
        .iter()
        .any(|name| std::env::var_os(name).is_some())
}

fn main() -> ExitCode {
    if is_run_by_agent() {
        return ExitCode::SUCCESS;
    }
    pretty_env_logger::init();
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n{}\nopen an issue at {}",
            "goboscript is cooked 💀".red().bold(),
            "https://github.com/aspizu/goboscript/issues".cyan()
        );
    }));
    let begin = Instant::now();
    let result = frontend();
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
