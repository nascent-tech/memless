use super::lower_filter::lower_filter;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn lower_selection(selection: &Option<Expr>, has_join: bool) -> Result<Option<Filter>, QueryRefusal> {
    match selection {
        Some(expr) => lower_filter(expr, has_join).map(Some),
        None => Ok(None),
    }
}
