use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn reject_locks(query: &Query) -> Result<(), QueryRefusal> {
    if !query.locks.is_empty() || query.for_clause.is_some() {
        return Err(outside("FOR UPDATE"));
    }
    if query.settings.is_some() || query.format_clause.is_some() {
        return Err(outside("SETTINGS"));
    }
    Ok(())
}
