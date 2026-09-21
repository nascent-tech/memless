use crate::base::table::Table;

pub(crate) fn next_position(tables: &[Table], name: &str) -> usize {
    tables
        .iter()
        .find(|table| table.name == name)
        .map(|table| table.rows.len() + 1)
        .unwrap_or(1)
}
