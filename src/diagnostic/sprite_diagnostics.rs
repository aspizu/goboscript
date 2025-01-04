use std::{io, path::PathBuf};

use annotate_snippets::{Annotation, Level, Renderer, Snippet};
use colored::Colorize;
use logos::Span;

use super::{diagnostic_kind::DiagnosticKind, Diagnostic};
use crate::{ast::Project, preproc::PreProc};

pub struct SpriteDiagnostics {
    pub path: PathBuf,
    pub preproc: PreProc,
    pub diagnostics: Vec<Diagnostic>,
}

impl SpriteDiagnostics {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let mut preproc = PreProc::new(path.parent().unwrap().to_path_buf());
        preproc.include(path.clone())?;
        preproc.process()?;
        Ok(Self {
            path,
            preproc,
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
        let src = self.preproc.get_translation_unit();
        for diagnostic in &self.diagnostics {
            let level: Level = (&diagnostic.kind).into();
            let title = diagnostic.kind.to_string(project, self);
            let help = diagnostic.kind.help(project, self);
            let help = help.as_ref();
            let (start, include) = self.preproc.translate_position(diagnostic.span.start);
            if diagnostic.span.start == 0 && diagnostic.span.end == 0 {
                let mut message = level.title(&title).snippet(
                    Snippet::source(&src[include.range.clone()])
                        .origin(include.path.to_str().unwrap())
                        .fold(true),
                );
                if let Some(help) = help {
                    message = message.footer(Level::Help.title(help));
                }
                eprintln!("{}", renderer.render(message));
            } else {
                let (end, _) = self.preproc.translate_position(diagnostic.span.end - 1);
                let end = end + 1;
                let mut message = level.title(&title).snippet(
                    Snippet::source(&src[include.range.clone()])
                        .origin(include.path.to_str().unwrap())
                        .fold(true)
                        .annotation(level.span(start..end)),
                );
                if let Some(help) = help {
                    message = message.footer(Level::Help.title(help));
                }
                eprintln!("{}", renderer.render(message));
            }
            if let DiagnosticKind::CommandFailed { stderr } = &diagnostic.kind {
                eprintln!("{}:", "stderr".red().bold());
                for line in stderr.split(|&b| b == b'\n') {
                    eprintln!("    {}", std::str::from_utf8(line).unwrap().red());
                }
            }
        }
    }
}
