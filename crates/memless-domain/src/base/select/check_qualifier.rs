use super::known_table::known_table;
use crate::base::table::Table;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::UnknownTable;

pub(crate) fn check_qualifier(from: &Table, joined: Option<&Table>, column: &ColumnRef) -> Result<(), QueryRefusal> {
    let Some(name) = &column.table else {
        return Ok(());
    };
    if known_table(from, joined, name) {
        return Ok(());
    }
    Err(UnknownTable { table: name.clone() })
}
