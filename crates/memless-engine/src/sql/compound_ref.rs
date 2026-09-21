use super::outside::outside;
use memless_domain::query::ColumnRef;
use memless_domain::QueryRefusal;
use sqlparser::ast::Ident;

pub(crate) fn compound_ref(parts: &[Ident]) -> Result<ColumnRef, QueryRefusal> {
    if parts.len() != 2 {
        return Err(outside("deeply qualified column"));
    }
    Ok(ColumnRef { table: Some(parts[0].value.clone()), column: parts[1].value.clone() })
}
