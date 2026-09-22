use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn reject_ctes(query: &Query) -> Result<(), QueryRefusal> {
    if query.with.is_some() {
        return Err(outside("WITH"));
    }
    Ok(())
}
