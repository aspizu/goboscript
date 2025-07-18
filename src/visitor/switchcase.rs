use logos::Span;

use crate::{
    ast::{
        Case,
        Expr,
        Stmt,
        Value,
    },
    blocks::{
        BinOp,
        UnOp,
    },
};

fn casearm(value: &Expr, cases: &[Case], span: &Span, index: usize) -> Stmt {
    Stmt::Branch {
        cond: Box::new(BinOp::Eq.to_expr(span.clone(), value.clone(), *cases[index].value.clone())),
        if_body: cases[index].body.clone(),
        else_body: if index < cases.len() - 1 {
            vec![casearm(value, cases, span, index + 1)]
        } else {
            vec![]
        },
    }
}

fn get_number(expr: &Expr) -> f64 {
    if let Expr::Value {
        value: Value::Number(number),
        ..
    } = expr
    {
        return *number;
    }
    unreachable!()
}

fn searcharm(
    value: &Expr,
    cases: &[Case],
    span: &Span,
    nums: &[(usize, f64)],
    low: usize,
    high: usize,
) -> Stmt {
    let mid = low + (1 + high - low) / 2;
    Stmt::Branch {
        cond: Box::new(BinOp::Lt.to_expr(
            span.clone(),
            value.clone(),
            Value::from(nums[mid].1).to_expr(span.clone()),
        )),
        if_body: if mid - low == 1 {
            cases[nums[low].0].body.clone()
        } else {
            vec![searcharm(value, cases, span, nums, low, mid - 1)]
        },
        else_body: if mid == high {
            cases[nums[mid].0].body.clone()
        } else {
            vec![searcharm(value, cases, span, nums, mid, high)]
        },
    }
}

fn searchtree(value: &Expr, cases: &[Case], span: &Span) -> Stmt {
    let mut nums = cases
        .iter()
        .enumerate()
        .map(|(i, case)| (i, get_number(&case.value)))
        .collect::<Vec<_>>();
    nums.sort_by(|a, b| a.1.total_cmp(&b.1));
    let low_minus_1 = nums[0].1 - 1.0;
    let high_plus_1 = nums[nums.len() - 1].1 + 1.0;
    Stmt::Branch {
        cond: Box::new(BinOp::And.to_expr(
            span.clone(),
            BinOp::And.to_expr(
                span.clone(),
                BinOp::Lt.to_expr(
                    span.clone(),
                    Value::from(low_minus_1).to_expr(span.clone()),
                    value.clone(),
                ),
                BinOp::Lt.to_expr(
                    span.clone(),
                    value.clone(),
                    Value::from(high_plus_1).to_expr(span.clone()),
                ),
            ),
            BinOp::Eq.to_expr(
                span.clone(),
                value.clone(),
                UnOp::Round.to_expr(span.clone(), value.clone()),
            ),
        )),
        if_body: vec![searcharm(value, cases, span, &nums, 0, nums.len() - 1)],
        else_body: vec![],
    }
}

pub fn switchcase(value: &Expr, cases: &[Case], span: &Span) -> Stmt {
    let all_integers = cases.iter().all(|case| {
        matches!(
            *case.value,
            Expr::Value {value: Value::Number(n),..
            } if n.fract() == 0.0
        )
    });
    // <25 doesn't benefit from search tree
    if cases.len() > 25 && all_integers {
        searchtree(value, cases, span)
    } else {
        casearm(value, cases, span, 0)
    }
}
