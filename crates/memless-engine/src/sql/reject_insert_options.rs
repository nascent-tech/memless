use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Insert as SqlInsert;

pub(crate) fn reject_insert_options(insert: &SqlInsert) -> Result<(), QueryRefusal> {
    if insert.on.is_some() {
        return Err(outside("ON CONFLICT"));
    }
    if insert.returning.is_some() {
        return Err(outside("RETURNING"));
    }
    if !insert.assignments.is_empty() {
        return Err(outside("INSERT with SET"));
    }
    Ok(())
}
