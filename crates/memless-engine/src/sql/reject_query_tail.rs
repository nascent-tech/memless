use super::reject_ctes::reject_ctes;
use super::reject_locks::reject_locks;
use super::reject_paging::reject_paging;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn reject_query_tail(query: &Query) -> Result<(), QueryRefusal> {
    reject_ctes(query)?;
    reject_paging(query)?;
    reject_locks(query)
}
