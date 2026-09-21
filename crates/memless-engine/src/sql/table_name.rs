use super::object_name::object_name;
use super::outside::outside;
use super::plain_table::plain_table;
use super::table_label::table_label;
use memless_domain::QueryRefusal;
use sqlparser::ast::TableFactor;

pub(crate) fn table_name(relation: &TableFactor) -> Result<String, QueryRefusal> {
    match plain_table(relation) {
        Some(name) => object_name(name),
        None => Err(outside(table_label(relation))),
    }
}
