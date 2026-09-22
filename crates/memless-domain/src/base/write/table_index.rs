use crate::base::table::Table;
use crate::refusal::QueryRefusal::UnknownTable;
use crate::refusal::Refusal;

pub(crate) fn table_index(tables: &[Table], name: &str) -> Result<usize, Refusal> {
    tables
        .iter()
        .position(|table| table.name == name)
        .ok_or_else(|| Refusal::from(UnknownTable { table: name.to_string() }))
}
