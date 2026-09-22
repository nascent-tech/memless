use super::task::Task;
use sqlparser::ast::{BinaryOperator, Expr};

pub(crate) fn boolean_parts<'e>(expr: &'e Expr) -> Option<(Task<'e>, &'e Expr, &'e Expr)> {
    match expr {
        Expr::BinaryOp { left, op: BinaryOperator::And, right } => Some((Task::CombineAnd, left, right)),
        Expr::BinaryOp { left, op: BinaryOperator::Or, right } => Some((Task::CombineOr, left, right)),
        _ => None,
    }
}
