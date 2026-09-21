use super::has_column::has_column;
use super::where_qualifier::where_qualifier;
use crate::base::table::Table;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal::UnknownColumn;
use crate::refusal::Refusal;

pub(crate) fn where_column(table: &Table, column: &ColumnRef) -> Result<(), Refusal> {
    where_qualifier(table, column)?;
    if has_column(table, &column.column) {
        return Ok(());
    }
    Err(Refusal::from(UnknownColumn { table: table.name.clone(), column: column.column.clone() }))
}
