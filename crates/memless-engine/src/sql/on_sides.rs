use super::column_ref_of::column_ref_of;
use memless_domain::query::ColumnRef;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn on_sides(left: &Expr, right: &Expr) -> Result<(ColumnRef, ColumnRef), QueryRefusal> {
    Ok((column_ref_of(left, true)?, column_ref_of(right, true)?))
}
