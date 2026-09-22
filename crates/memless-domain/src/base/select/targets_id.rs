use super::is_id_of::is_id_of;
use crate::base::table::Table;
use crate::query::ColumnRef;
use crate::relation::guessed_target;

pub(crate) fn targets_id(column: &str, other: &Table, id: &ColumnRef) -> bool {
    guessed_target(column).as_deref() == Some(other.name.as_str()) && is_id_of(other, id)
}
