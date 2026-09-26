use super::direction_of::direction_of;
use super::order_column::order_column;
use super::reject_nulls::reject_nulls;
use memless_domain::query::OrderKey;
use memless_domain::QueryRefusal;
use sqlparser::ast::OrderByExpr;

pub(crate) fn lower_order_key(expr: &OrderByExpr, has_join: bool) -> Result<OrderKey, QueryRefusal> {
    reject_nulls(expr)?;
    let column = order_column(&expr.expr, has_join)?;
    Ok(OrderKey { column, direction: direction_of(expr.asc) })
}
