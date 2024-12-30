use logos::Span;
use smol_str::SmolStr;

use super::{expr::Expr, Event, Stmt};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum EventKind {
    OnFlag,
    OnKey { key: SmolStr, span: Span },
    OnClick,
    OnBackdrop { backdrop: SmolStr, span: Span },
    OnLoudnessGt { value: Box<Expr> },
    OnTimerGt { value: Box<Expr> },
    OnClone,
}

impl EventKind {
    pub fn opcode(&self) -> &'static str {
        match &self {
            EventKind::OnFlag => "event_whenflagclicked",
            EventKind::OnKey { .. } => "event_whenkeypressed",
            EventKind::OnClick => "event_whenthisspriteclicked",
            EventKind::OnBackdrop { .. } => "event_whenbackdropswitchesto",
            EventKind::OnLoudnessGt { .. } | EventKind::OnTimerGt { .. } => "event_whengreaterthan",
            EventKind::OnClone => "control_start_as_clone",
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_event(self, span: Span, body: Vec<Stmt>) -> Event {
        Event {
            kind: self,
            body,
            span,
        }
    }
}
