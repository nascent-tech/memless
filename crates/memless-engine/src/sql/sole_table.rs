use super::outside::outside;
use super::table_name::table_name;
use memless_domain::QueryRefusal;
use sqlparser::ast::TableWithJoins;

pub(crate) fn sole_table(table: &TableWithJoins) -> Result<String, QueryRefusal> {
    if !table.joins.is_empty() {
        return Err(outside("a write with a join"));
    }
    table_name(&table.relation)
}
