use super::column_ref_of::column_ref_of;
use super::compare_leaf::compare_leaf;
use super::outside::outside;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn lower_leaf(expr: &Expr, has_join: bool) -> Result<Filter, QueryRefusal> {
    match expr {
        Expr::IsNull(inner) => Ok(Filter::IsNull(column_ref_of(inner, has_join)?)),
        Expr::IsNotNull(inner) => Ok(Filter::IsNotNull(column_ref_of(inner, has_join)?)),
        Expr::BinaryOp { left, op, right } => compare_leaf(left, op, right, has_join),
        _ => Err(outside("this condition")),
    }
}
