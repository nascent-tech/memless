use super::check_qualifier::check_qualifier;
use super::has_column::has_column;
use super::target_table::target_table;
use crate::base::table::Table;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::UnknownColumn;

pub(crate) fn check_one_column(from: &Table, joined: Option<&Table>, column: &ColumnRef) -> Result<(), QueryRefusal> {
    check_qualifier(from, joined, column)?;
    let table = target_table(from, joined, column);
    if has_column(table, &column.column) {
        return Ok(());
    }
    Err(UnknownColumn { table: table.name.clone(), column: column.column.clone() })
}
