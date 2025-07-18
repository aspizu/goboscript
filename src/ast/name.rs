use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        is_generated: bool,
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

    pub fn is_generated(&self) -> bool {
        match self {
            Self::Name { .. } => false,
            Self::DotName { is_generated, .. } => *is_generated,
        }
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Name::Name { name: name1, .. }, Name::Name { name: name2, .. }) => name1 == name2,
            (
                Name::DotName {
                    lhs: lhs1,
                    rhs: rhs1,
                    ..
                },
                Name::DotName {
                    lhs: lhs2,
                    rhs: rhs2,
                    ..
                },
            ) => lhs1 == lhs2 && rhs1 == rhs2,
            _ => false,
        }
    }
}

impl Eq for Name {}
