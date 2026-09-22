use super::ident_name::ident_name;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::ObjectName;

pub(crate) fn object_name(name: &ObjectName) -> Result<String, QueryRefusal> {
    if name.0.len() != 1 {
        return Err(outside("qualified table name"));
    }
    Ok(ident_name(&name.0[0]))
}
