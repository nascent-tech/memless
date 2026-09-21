use super::matches_row::matches_row;
use crate::base::table::Table;
use crate::query::Delete;

pub(crate) fn cut_rows(table: &mut Table, spec: &Delete) -> u64 {
    let before = table.rows.len();
    table.rows.retain(|row| !matches_row(row, &spec.filter));
    (before - table.rows.len()) as u64
}
