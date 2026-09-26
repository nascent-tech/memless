use super::outside::outside;
use super::reject_order::reject_order;
use super::reject_query_tail::reject_query_tail;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, Query, SetExpr};

pub(crate) fn values_row(query: Query) -> Result<Vec<Expr>, QueryRefusal> {
    reject_query_tail(&query)?;
    reject_order(&query)?;
    let SetExpr::Values(values) = *query.body else {
        return Err(outside("INSERT ... SELECT"));
    };
    if values.explicit_row || values.rows.len() != 1 {
        return Err(outside("multi-row VALUES"));
    }
    Ok(values.rows.into_iter().next().unwrap_or_default())
}
