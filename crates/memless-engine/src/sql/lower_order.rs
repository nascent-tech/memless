use super::lower_order_key::lower_order_key;
use super::outside::outside;
use memless_domain::query::{Items, OrderKey};
use memless_domain::QueryRefusal;
use sqlparser::ast::OrderBy;

pub(crate) fn lower_order(
    order_by: &Option<OrderBy>,
    items: &Items,
    has_join: bool,
) -> Result<Vec<OrderKey>, QueryRefusal> {
    let Some(order_by) = order_by else {
        return Ok(Vec::new());
    };
    if matches!(items, Items::Aggregates(_)) {
        return Err(outside("ORDER BY with an aggregate"));
    }
    order_by.exprs.iter().map(|expr| lower_order_key(expr, has_join)).collect()
}
