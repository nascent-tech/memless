use super::column_ref::ColumnRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Join {
    pub table: String,
    pub left: ColumnRef,
    pub right: ColumnRef,
}
