use crate::query::ColumnRef;
use crate::scalar::Scalar;

pub(crate) trait Cells {
    fn cell(&self, column: &ColumnRef) -> Option<&Scalar>;
}
