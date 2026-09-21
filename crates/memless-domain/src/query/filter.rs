use super::column_ref::ColumnRef;
use super::compare::Compare;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Compare(Compare),
    IsNull(ColumnRef),
    IsNotNull(ColumnRef),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
}
