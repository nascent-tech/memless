use super::item_columns::item_columns;
use super::join_columns::join_columns;
use crate::base::filter::columns;
use crate::query::{ColumnRef, Select};

pub(crate) fn referenced_columns(query: &Select) -> Vec<&ColumnRef> {
    let mut references = join_columns(query);
    references.extend(item_columns(&query.items));
    if let Some(filter) = &query.filter {
        references.extend(columns(filter));
    }
    references.extend(query.order.iter().map(|key| &key.column));
    references
}
