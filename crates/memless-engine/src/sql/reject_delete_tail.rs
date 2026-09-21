use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Delete as SqlDelete;

pub(crate) fn reject_delete_tail(delete: &SqlDelete) -> Result<(), QueryRefusal> {
    if delete.using.is_some() {
        return Err(outside("DELETE USING"));
    }
    if delete.returning.is_some() {
        return Err(outside("RETURNING"));
    }
    if !delete.order_by.is_empty() || delete.limit.is_some() || !delete.tables.is_empty() {
        return Err(outside("DELETE with a tail clause"));
    }
    Ok(())
}
