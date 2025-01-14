use std::{
    env::consts::OS,
    fs::File,
    io::{
        BufRead,
        BufReader,
        Write,
    },
    path::Path,
    process::{
        Command,
        Stdio,
    },
};

use crate::{
    ast::Cmd,
    diagnostic::{
        Diagnostic,
        DiagnosticKind,
    },
};

pub fn cmd_to_list(cmd: &Cmd, input: &Path) -> Result<Vec<String>, Diagnostic> {
    if cmd
        .program
        .as_ref()
        .is_some_and(|program| &*program.name == "file")
    {
        let file = match File::open(input.join(&*cmd.cmd)) {
            Ok(file) => file,
            Err(error) => {
                return Err(Diagnostic {
                    kind: DiagnosticKind::IOError(error),
                    span: cmd.span.clone(),
                });
            }
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        return Ok(lines);
    }
    let mut child = if let Some(program) = &cmd.program {
        let command = Command::new(&*program.name)
            .current_dir(input)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        if let Err(error) = command {
            return Err(Diagnostic {
                kind: DiagnosticKind::IOError(error),
                span: program.span.clone(),
            });
        }
        command.unwrap()
    } else if OS == "windows" {
        unimplemented!()
    } else {
        Command::new("bash")
            .args(["-e", "-u", "-o", "pipefail"])
            .current_dir(input)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(cmd.cmd.as_bytes()).unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        let mut lines: Vec<String> = output.stdout.lines().map_while(Result::ok).collect();
        if lines.last().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }
        Ok(lines)
    } else {
        Err(Diagnostic {
            kind: DiagnosticKind::CommandFailed {
                stderr: output.stderr,
            },
            span: cmd.span.clone(),
        })
    }
}
