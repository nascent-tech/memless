use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn is_id_of(table: &Table, id: &ColumnRef) -> bool {
    id.column == "id" && id.table.as_deref() == Some(table.name.as_str())
}
