use std::{fs, io, path::PathBuf};

use annotate_snippets::{Level, Renderer, Snippet};
use colored::Colorize;
use logos::Span;

use super::{diagnostic_kind::DiagnosticKind, Diagnostic};
use crate::ast::Project;

pub struct SpriteDiagnostics {
    pub path: PathBuf,
    pub src: String,
    pub diagnostics: Vec<Diagnostic>,
}

impl SpriteDiagnostics {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let src = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            src,
            diagnostics: Vec::new(),
        })
    }

    pub fn report(&mut self, kind: DiagnosticKind, span: &Span) {
        self.diagnostics.push(Diagnostic {
            kind,
            span: span.clone(),
        });
    }

    pub fn eprint(&self, renderer: &Renderer, project: &Project) {
        for diagnostic in &self.diagnostics {
            let level: Level = (&diagnostic.kind).into();
            let title = diagnostic.kind.to_string(project, self);
            let message = level.title(&title).snippet(
                Snippet::source(&self.src)
                    .origin(self.path.to_str().unwrap())
                    .fold(true)
                    .annotation(level.span(diagnostic.span.clone())),
            );
            eprintln!("{}", renderer.render(message));
            if let DiagnosticKind::CommandFailed { stderr } = &diagnostic.kind {
                eprintln!("{}:", "stderr".red().bold());
                for line in stderr.split(|&b| b == b'\n') {
                    eprintln!("    {}", std::str::from_utf8(line).unwrap().red());
                }
            }
        }
    }
}
