use super::column_ref::ColumnRef;
use super::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderKey {
    pub column: ColumnRef,
    pub direction: Direction,
}
