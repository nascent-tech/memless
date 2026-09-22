use super::aggregate_column::aggregate_column;
use crate::query::{ColumnRef, Items};

pub(crate) fn item_columns(items: &Items) -> Vec<&ColumnRef> {
    match items {
        Items::All => Vec::new(),
        Items::Columns(columns) => columns.iter().collect(),
        Items::Aggregates(aggregates) => aggregates.iter().filter_map(aggregate_column).collect(),
    }
}
