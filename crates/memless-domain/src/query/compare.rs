use super::column_ref::ColumnRef;
use super::op::Op;
use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Compare {
    pub column: ColumnRef,
    pub op: Op,
    pub literal: Scalar,
}
