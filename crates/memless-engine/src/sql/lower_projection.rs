use super::assemble::assemble;
use super::classify::classify;
use super::is_wildcard::is_wildcard;
use memless_domain::query::Items;
use memless_domain::QueryRefusal;
use sqlparser::ast::SelectItem;

pub(crate) fn lower_projection(projection: &[SelectItem], has_join: bool) -> Result<Items, QueryRefusal> {
    if is_wildcard(projection) {
        return Ok(Items::All);
    }
    assemble(classify(projection, has_join)?)
}
