use logos::Span;
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub enum Name {
    Name {
        name: SmolStr,
        span: Span,
    },
    DotName {
        lhs: SmolStr,
        lhs_span: Span,
        rhs: SmolStr,
        rhs_span: Span,
    },
}

impl Name {
    pub fn span(&self) -> Span {
        match self {
            Self::Name { span, .. } => span.clone(),
            Self::DotName {
                lhs_span, rhs_span, ..
            } => lhs_span.start..rhs_span.end,
        }
    }

    pub fn basename(&self) -> &SmolStr {
        match self {
            Self::Name { name, .. } => name,
            Self::DotName { lhs, .. } => lhs,
        }
    }

    pub fn basespan(&self) -> Span {
        match self {
            Self::Name { span, .. } => span.clone(),
            Self::DotName { lhs_span, .. } => lhs_span.clone(),
        }
    }

    pub fn fieldname(&self) -> Option<&SmolStr> {
        match self {
            Self::Name { .. } => None,
            Self::DotName { rhs, .. } => Some(rhs),
        }
    }

    pub fn fieldspan(&self) -> Span {
        match self {
            Self::Name { span, .. } => span.clone(),
            Self::DotName { rhs_span, .. } => rhs_span.clone(),
        }
    }
}
