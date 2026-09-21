use super::col::Col;
use super::existing_columns::existing_columns;
use super::one_col::one_col;
use crate::base::table::Table;

pub(crate) fn table_cols(table: &Table, joined: bool, has_join: bool) -> Vec<Col> {
    existing_columns(table)
        .into_iter()
        .map(|name| one_col(&table.name, name, joined, has_join))
        .collect()
}
