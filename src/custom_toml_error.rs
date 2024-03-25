use std::{
    fmt::{Display, Formatter, Result},
    path::PathBuf,
};

use colored::Colorize;

fn translate_position(input: &[u8], index: usize) -> (usize, usize) {
    if input.is_empty() {
        return (0, index);
    }

    let safe_index = index.min(input.len() - 1);
    let column_offset = index - safe_index;
    let index = safe_index;

    let nl = input[0..index]
        .iter()
        .rev()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .map(|(nl, _)| index - nl - 1);
    let line_start = match nl {
        Some(nl) => nl + 1,
        None => 0,
    };
    let line = input[0..line_start].iter().filter(|b| **b == b'\n').count();

    let column = std::str::from_utf8(&input[line_start..=index])
        .map(|s| s.chars().count() - 1)
        .unwrap_or_else(|_| index - line_start);
    let column = column + column_offset;

    (line, column)
}

pub struct CustomTOMLError {
    path: PathBuf,
    src: String,
    inner: toml::de::Error,
}

impl CustomTOMLError {
    pub fn new(path: PathBuf, src: String, err: toml::de::Error) -> Self {
        Self { path, src, inner: err }
    }
}

impl Display for CustomTOMLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let span = self.inner.span().unwrap();
        let (line, column) = translate_position(self.src.as_bytes(), span.start);
        let line_num = line + 1;
        let col_num = column + 1;
        let content = self.src.split('\n').nth(line).expect("valid line number");
        writeln!(
            f,
            "{}{} {}",
            "error".bold().red(),
            ":".bold(),
            "TOML parse error".bold()
        )?;
        writeln!(
            f,
            "  {} {}:{}:{}",
            "-->".bold(),
            self.path.to_str().unwrap().blue(),
            line_num,
            col_num
        )?;
        writeln!(f, "{}", "      |".bold())?;
        writeln!(f, "{} {}", format!(" {:4} |", line_num).bold(), content)?;
        writeln!(
            f,
            "{} {}{}",
            "      |".bold(),
            " ".repeat(col_num - 1),
            "^".repeat(span.len()).bold().red()
        )
    }
}
