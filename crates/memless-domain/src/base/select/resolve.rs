use crate::base::table::Table;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::UnknownTable;

pub(crate) fn find_table<'a>(tables: &'a [Table], name: &str) -> Result<&'a Table, QueryRefusal> {
    tables
        .iter()
        .find(|table| table.name == name)
        .ok_or_else(|| UnknownTable { table: name.to_string() })
}
