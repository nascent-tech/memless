use super::row_cells::RowCells;
use crate::base::filter::keeps;
use crate::base::row::Row;
use crate::query::Filter;

pub(crate) fn matches_row(row: &Row, filter: &Option<Filter>) -> bool {
    match filter {
        None => true,
        Some(filter) => keeps(&RowCells { row }, filter),
    }
}
