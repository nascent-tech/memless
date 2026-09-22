use super::filter_columns::filter_columns;
use super::item_columns::item_columns;
use super::join_columns::join_columns;
use crate::query::{ColumnRef, Select};

pub(crate) fn referenced_columns(query: &Select) -> Vec<&ColumnRef> {
    let mut columns = join_columns(query);
    columns.extend(item_columns(&query.items));
    if let Some(filter) = &query.filter {
        columns.extend(filter_columns(filter));
    }
    columns
}
