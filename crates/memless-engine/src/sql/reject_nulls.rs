use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::OrderByExpr;

pub(crate) fn reject_nulls(expr: &OrderByExpr) -> Result<(), QueryRefusal> {
    match expr.nulls_first {
        Some(true) => Err(outside("NULLS FIRST")),
        Some(false) => Err(outside("NULLS LAST")),
        None => Ok(()),
    }
}
