use super::push_unique::push_unique;
use crate::base::table::Table;

pub(crate) fn existing_columns(table: &Table) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    table.rows.iter().flat_map(|row| row.columns.iter()).for_each(|(key, _)| push_unique(&mut seen, key));
    seen
}
