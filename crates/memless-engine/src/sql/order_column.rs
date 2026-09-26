use super::column_ref_of::column_ref_of;
use super::outside::outside;
use memless_domain::query::ColumnRef;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, Value};

pub(crate) fn order_column(expr: &Expr, has_join: bool) -> Result<ColumnRef, QueryRefusal> {
    match expr {
        Expr::Identifier(_) | Expr::CompoundIdentifier(_) => column_ref_of(expr, has_join),
        Expr::Value(Value::Number(..)) => Err(outside("ORDER BY position")),
        _ => Err(outside("ORDER BY expression")),
    }
}
