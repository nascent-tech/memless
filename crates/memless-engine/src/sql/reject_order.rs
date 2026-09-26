use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn reject_order(query: &Query) -> Result<(), QueryRefusal> {
    match query.order_by {
        Some(_) => Err(outside("ORDER BY")),
        None => Ok(()),
    }
}
