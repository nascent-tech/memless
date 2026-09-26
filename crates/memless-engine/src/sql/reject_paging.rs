use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn reject_paging(query: &Query) -> Result<(), QueryRefusal> {
    if query.limit.is_some() || !query.limit_by.is_empty() {
        return Err(outside("LIMIT"));
    }
    if query.offset.is_some() {
        return Err(outside("OFFSET"));
    }
    if query.fetch.is_some() {
        return Err(outside("FETCH"));
    }
    Ok(())
}
