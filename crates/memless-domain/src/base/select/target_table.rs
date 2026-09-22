use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn target_table<'a>(from: &'a Table, joined: Option<&'a Table>, column: &ColumnRef) -> &'a Table {
    match (&column.table, joined) {
        (Some(name), Some(table)) if *name == table.name => table,
        _ => from,
    }
}
