use memless_domain::query::Direction;

pub(crate) fn direction_of(asc: Option<bool>) -> Direction {
    match asc {
        Some(false) => Direction::Descending,
        _ => Direction::Ascending,
    }
}
