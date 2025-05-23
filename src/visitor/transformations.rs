use fxhash::FxHashMap;
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
    let mut field = field.value.as_ref().clone();
    if let Expr::Name(Name::DotName { is_generated, .. }) = &mut field {
        *is_generated = false;
    }
    Some(field)
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
                            is_generated: true,
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
                    is_generated: true,
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
                    is_generated: true,
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
    Some(Value::bin_op(*op, lhs_value, rhs_value).to_expr(span.clone()))
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
    Some(Value::un_op(*op, opr_value).to_expr(span.clone()))
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
        Value::from(0.0).to_expr(span.clone()),
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

pub fn keyword_arguments(
    signature: Option<&Vec<Arg>>,
    args: &mut Vec<Expr>,
    kwargs: &mut FxHashMap<SmolStr, (Span, Expr)>,
    _d: D, // currently not used in this implementation
) {
    if let Some(sig) = signature {
        // Build a new vector of arguments in the order given by the signature.
        let mut new_args = Vec::with_capacity(sig.len());
        let mut pos = 0;

        for param in sig {
            if pos < args.len() {
                // If there is both a positional and keyword argument, we prefer the positional one.
                // Remove the keyword argument from the map.
                kwargs.remove(&param.name);
                // Use the next positional argument.
                new_args.push(args[pos].clone());
                pos += 1;
            } else if let Some((_, kw_expr)) = kwargs.remove(&param.name) {
                // No more positional args, but there is a matching keyword argument.
                new_args.push(kw_expr);
            } else if let Some((default, span)) = &param.default {
                // Compute the default value if one is provided.
                new_args.push(default.clone().to_expr(span.clone()));
            }
            // If no positional, keyword, or default value exists, then
            // we simply do not insert anything (and no error is raised).
        }

        // Append any extra positional arguments that exceed the signature length.
        while pos < args.len() {
            new_args.push(args[pos].clone());
            pos += 1;
        }

        // Replace the original args with the re-ordered version.
        *args = new_args;
    }
    assert!(kwargs.is_empty(), "kwargs should be empty after processing");
}
