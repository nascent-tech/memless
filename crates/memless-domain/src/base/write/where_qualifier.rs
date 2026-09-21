use crate::base::table::Table;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal::UnknownTable;
use crate::refusal::Refusal;

pub(crate) fn where_qualifier(table: &Table, column: &ColumnRef) -> Result<(), Refusal> {
    let Some(name) = &column.table else {
        return Ok(());
    };
    if name == &table.name {
        return Ok(());
    }
    Err(Refusal::from(UnknownTable { table: name.clone() }))
}
