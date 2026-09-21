use super::aggregate::Aggregate;
use super::column_ref::ColumnRef;

#[derive(Debug, Clone, PartialEq)]
pub enum Items {
    All,
    Columns(Vec<ColumnRef>),
    Aggregates(Vec<Aggregate>),
}
