use super::lower_leaf::lower_leaf;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn push_leaf(output: &mut Vec<Filter>, expr: &Expr, has_join: bool) -> Result<(), QueryRefusal> {
    output.push(lower_leaf(expr, has_join)?);
    Ok(())
}
