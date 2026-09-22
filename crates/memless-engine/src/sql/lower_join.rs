use super::one_join::one_join;
use super::outside::outside;
use memless_domain::query::Join;
use memless_domain::QueryRefusal;
use sqlparser::ast::Join as SqlJoin;

pub(crate) fn lower_join(joins: &[SqlJoin], from_name: &str) -> Result<Option<Join>, QueryRefusal> {
    if joins.is_empty() {
        return Ok(None);
    }
    if joins.len() > 1 {
        return Err(outside("multiple joins"));
    }
    one_join(&joins[0], from_name).map(Some)
}
