use crate::query::{Aggregate, ColumnRef};

pub(crate) fn aggregate_column(aggregate: &Aggregate) -> Option<&ColumnRef> {
    match aggregate {
        Aggregate::CountStar => None,
        Aggregate::Count(column) | Aggregate::Sum(column) => Some(column),
    }
}
