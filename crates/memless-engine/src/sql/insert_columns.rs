use super::ident_name::ident_name;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Insert as SqlInsert;

pub(crate) fn insert_columns(insert: &SqlInsert) -> Result<Vec<String>, QueryRefusal> {
    if insert.columns.is_empty() {
        return Err(outside("INSERT without a column list"));
    }
    Ok(insert.columns.iter().map(ident_name).collect())
}
