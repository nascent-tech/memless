use super::on_sides::on_sides;
use super::outside::outside;
use memless_domain::query::ColumnRef;
use memless_domain::QueryRefusal;
use sqlparser::ast::{BinaryOperator, Expr};

pub(crate) fn lower_on(expr: &Expr) -> Result<(ColumnRef, ColumnRef), QueryRefusal> {
    match expr {
        Expr::BinaryOp { left, op: BinaryOperator::Eq, right } => on_sides(left, right),
        _ => Err(outside("this JOIN condition")),
    }
}
