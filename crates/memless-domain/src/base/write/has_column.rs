use crate::base::table::Table;

pub(crate) fn has_column(table: &Table, name: &str) -> bool {
    table.rows.iter().any(|row| row.columns.iter().any(|(key, _)| key == name))
}
