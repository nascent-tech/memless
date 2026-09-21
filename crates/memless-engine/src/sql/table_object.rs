use super::object_name::object_name;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::TableObject;

pub(crate) fn table_object_name(table: &TableObject) -> Result<String, QueryRefusal> {
    match table {
        TableObject::TableName(name) => object_name(name),
        TableObject::TableFunction(_) => Err(outside("INSERT into a table function")),
    }
}
