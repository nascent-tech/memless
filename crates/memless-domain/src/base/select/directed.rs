use std::cmp::Ordering;

use crate::query::Direction;

pub(crate) fn directed(direction: Direction, ordering: Option<Ordering>) -> Ordering {
    let ordering = ordering.unwrap_or(Ordering::Equal);
    match direction {
        Direction::Ascending => ordering,
        Direction::Descending => ordering.reverse(),
    }
}
