use crate::base::filter::Cells;
use crate::base::row::Row;
use crate::query::ColumnRef;
use crate::scalar::Scalar;

pub(crate) struct RowCells<'a> {
    pub row: &'a Row,
}

impl Cells for RowCells<'_> {
    fn cell(&self, column: &ColumnRef) -> Option<&Scalar> {
        self.row.columns.iter().find(|(key, _)| key == &column.column).map(|(_, value)| value)
    }
}
