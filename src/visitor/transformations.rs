use logos::Span;

use super::pass2::S;
use crate::{
    ast::{
        Arg,
        Expr,
        Name,
        StructLiteralField,
        Value,
    },
    blocks::{
        BinOp,
        Repr,
        UnOp,
    },
    codegen::sb3::D,
    diagnostic::DiagnosticKind,
    misc::SmolStr,
};

pub fn apply<T, F>(value: &mut T, transformer: F)
where F: FnOnce(&T) -> Option<T> {
    let replace = transformer(value);
    if let Some(replace) = replace {
        *value = replace;
    }
}

pub fn struct_literal_field_access(expr: &Expr, d: D) -> Option<Expr> {
    let Expr::Dot { lhs, rhs, rhs_span } = expr else {
        return None;
    };
    let Expr::StructLiteral {
        name: lhs_name,
        span: _,
        fields,
    } = lhs.as_ref()
    else {
        return None;
    };
    let Some(field) = fields.iter().find(|field| &field.name == rhs) else {
        d.report(
            DiagnosticKind::StructDoesNotHaveField {
                type_name: lhs_name.clone(),
                field_name: rhs.clone(),
            },
            rhs_span,
        );
        return None;
    };
    Some(field.value.as_ref().clone())
}

pub fn list_field_access(expr: &Expr, s: S) -> Option<Expr> {
    let Expr::BinOp {
        op: BinOp::Of,
        lhs,
        rhs,
        ..
    } = expr
    else {
        return None;
    };
    let Expr::Name(Name::Name { name, span }) = lhs.as_ref() else {
        return None;
    };
    let list = s.get_list(name)?;
    let (type_name, type_span) = list.type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(Expr::StructLiteral {
        name: type_name.clone(),
        span: type_span.clone(),
        fields: struct_
            .fields
            .iter()
            .map(|field| StructLiteralField {
                name: field.name.clone(),
                span: field.span.clone(),
                value: BinOp::Of
                    .to_expr(
                        span.clone(),
                        Expr::Name(Name::DotName {
                            lhs: name.clone(),
                            lhs_span: span.clone(),
                            rhs: field.name.clone(),
                            rhs_span: field.span.clone(),
                        }),
                        rhs.as_ref().clone(),
                    )
                    .into(),
            })
            .collect(),
    })
}

pub fn variable_field_access(expr: &Expr, s: S) -> Option<Expr> {
    let Expr::Name(name) = expr else {
        return None;
    };
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let span = name.span();
    let var = &s.get_var(basename)?;
    let (type_name, type_span) = var.type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(Expr::StructLiteral {
        name: type_name.clone(),
        span: type_span.clone(),
        fields: struct_
            .fields
            .iter()
            .map(|field| StructLiteralField {
                name: field.name.clone(),
                span: field.span.clone(),
                value: Expr::Name(Name::DotName {
                    lhs: var.name.clone(),
                    lhs_span: span.clone(),
                    rhs: field.name.clone(),
                    rhs_span: field.span.clone(),
                })
                .into(),
            })
            .collect(),
    })
}

pub fn arg_field_access(expr: &Expr, s: S) -> Option<Expr> {
    let Expr::Arg(name) = expr else {
        return None;
    };
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let span = name.span();
    let arg = s.args?.iter().find(|arg| &arg.name == basename)?;
    let (type_name, type_span) = arg.type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(Expr::StructLiteral {
        name: type_name.clone(),
        span: type_span.clone(),
        fields: struct_
            .fields
            .iter()
            .map(|field| StructLiteralField {
                name: field.name.clone(),
                span: field.span.clone(),
                value: Expr::Arg(Name::DotName {
                    lhs: arg.name.clone(),
                    lhs_span: span.clone(),
                    rhs: field.name.clone(),
                    rhs_span: field.span.clone(),
                })
                .into(),
            })
            .collect(),
    })
}

pub fn bin_op(expr: &Expr) -> Option<Expr> {
    let Expr::BinOp { op, span, lhs, rhs } = expr else {
        return None;
    };
    let Expr::Value {
        value: lhs_value, ..
    } = lhs.as_ref()
    else {
        return None;
    };
    let Expr::Value {
        value: rhs_value, ..
    } = rhs.as_ref()
    else {
        return None;
    };
    lhs_value
        .binop(*op, rhs_value)
        .map(|value| value.to_expr(span.clone()))
}

pub fn un_op(expr: &Expr) -> Option<Expr> {
    let Expr::UnOp { op, span, opr } = expr else {
        return None;
    };
    let Expr::Value {
        value: opr_value, ..
    } = opr.as_ref()
    else {
        return None;
    };
    opr_value.unop(*op).map(|value| value.to_expr(span.clone()))
}

pub fn minus(expr: &Expr) -> Option<Expr> {
    let Expr::UnOp {
        op: UnOp::Minus,
        span,
        opr,
    } = expr
    else {
        return None;
    };
    Some(BinOp::Sub.to_expr(
        span.clone(),
        Value::Int(0).to_expr(span.clone()),
        opr.as_ref().clone(),
    ))
}

pub fn less_than_equal(expr: &Expr) -> Option<Expr> {
    let Expr::BinOp {
        op: BinOp::Le,
        span,
        lhs,
        rhs,
    } = expr
    else {
        return None;
    };
    Some(UnOp::Not.to_expr(
        span.clone(),
        BinOp::Gt.to_expr(span.clone(), lhs.as_ref().clone(), rhs.as_ref().clone()),
    ))
}

pub fn greater_than_equal(expr: &Expr) -> Option<Expr> {
    let Expr::BinOp {
        op: BinOp::Ge,
        span,
        lhs,
        rhs,
    } = expr
    else {
        return None;
    };
    Some(UnOp::Not.to_expr(
        span.clone(),
        BinOp::Lt.to_expr(span.clone(), lhs.as_ref().clone(), rhs.as_ref().clone()),
    ))
}

pub fn not_equal(expr: &Expr) -> Option<Expr> {
    let Expr::BinOp {
        op: BinOp::Ne,
        span,
        lhs,
        rhs,
    } = expr
    else {
        return None;
    };
    Some(UnOp::Not.to_expr(
        span.clone(),
        BinOp::Eq.to_expr(span.clone(), lhs.as_ref().clone(), rhs.as_ref().clone()),
    ))
}

pub fn floor_div(expr: &Expr) -> Option<Expr> {
    let Expr::BinOp {
        op: BinOp::FloorDiv,
        span,
        lhs,
        rhs,
    } = expr
    else {
        return None;
    };
    Some(UnOp::Floor.to_expr(
        span.clone(),
        BinOp::Div.to_expr(span.clone(), lhs.as_ref().clone(), rhs.as_ref().clone()),
    ))
}

pub fn coerce_condition(expr: &Expr) -> Option<Expr> {
    if matches!(
        expr,
        Expr::UnOp { op: UnOp::Not, .. }
            | Expr::BinOp {
                op: BinOp::Eq
                    | BinOp::Ne
                    | BinOp::Lt
                    | BinOp::Le
                    | BinOp::Gt
                    | BinOp::Ge
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::In,
                ..
            }
            | Expr::Repr {
                repr: Repr::ColorIsTouchingColor
                    | Repr::KeyPressed
                    | Repr::MouseDown
                    | Repr::Touching
                    | Repr::TouchingColor
                    | Repr::TouchingEdge
                    | Repr::TouchingMousePointer
                    | Repr::Contains,
                ..
            }
    ) {
        return None;
    }
    Some(BinOp::Eq.to_expr(
        expr.span(),
        expr.clone(),
        Value::Int(1).to_expr(expr.span()),
    ))
}

pub fn keyword_arguments(
    args: &mut Vec<(Option<(SmolStr, Span)>, Expr)>,
    arg_names: Option<&Vec<Arg>>,
    d: D,
) {
    let mut i = 0;
    if let Some(arg_names) = arg_names {
        for arg in arg_names {
            if let Some(index) = args.iter().position(|(arg_name, _)| {
                arg_name
                    .as_ref()
                    .is_some_and(|(arg_name, _)| *arg_name == arg.name)
            }) {
                let arg = args.remove(index);
                args.insert(i, (None, arg.1));
            } else if let Some(index) = args[i..]
                .iter()
                .position(|(arg_name, _)| arg_name.is_none())
            {
                let arg = args.remove(i + index);
                args.insert(i, arg);
            }
            i += 1;
        }
    }
    for (arg_name, _) in args {
        if let Some((arg_name, arg_span)) = arg_name.take() {
            d.report(DiagnosticKind::UnrecognizedArgument(arg_name), &arg_span);
        }
    }
}
