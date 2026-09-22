use super::column_ref::ColumnRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Aggregate {
    CountStar,
    Count(ColumnRef),
    Sum(ColumnRef),
}
