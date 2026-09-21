use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Insert as SqlInsert;

pub(crate) fn reject_insert_shape(insert: &SqlInsert) -> Result<(), QueryRefusal> {
    if insert.or.is_some() || insert.overwrite || insert.replace_into {
        return Err(outside("INSERT with a conflict modifier"));
    }
    if !insert.into || insert.has_table_keyword {
        return Err(outside("INSERT ... TABLE"));
    }
    Ok(())
}
