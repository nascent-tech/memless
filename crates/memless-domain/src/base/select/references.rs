use super::row_column::row_column;
use super::same_id::same_id;
use crate::base::row::Row;
use crate::Id;

pub(crate) fn references(holder: &Row, column: &str, target: &Id) -> bool {
    match row_column(holder, column) {
        Some(value) => same_id(value, target),
        None => false,
    }
}
