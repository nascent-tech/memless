use super::lower_select::lower_select;
use super::reject_query_tail::reject_query_tail;
use super::select_of::select_of;
use memless_domain::query::Select;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn lower_query(query: Box<Query>) -> Result<Select, QueryRefusal> {
    reject_query_tail(&query)?;
    let Query { body, order_by, .. } = *query;
    let select = select_of(*body)?;
    lower_select(&select, &order_by)
}
