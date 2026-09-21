use super::single_from::single_from;
use super::sole_table::sole_table;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Delete as SqlDelete, FromTable};

pub(crate) fn delete_table(delete: &SqlDelete) -> Result<String, QueryRefusal> {
    let tables = match &delete.from {
        FromTable::WithFromKeyword(tables) => tables,
        FromTable::WithoutKeyword(tables) => tables,
    };
    sole_table(single_from(tables)?)
}
