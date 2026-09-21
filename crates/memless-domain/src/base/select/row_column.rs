use crate::base::row::Row;
use crate::scalar::Scalar;

pub(crate) fn row_column<'a>(row: &'a Row, name: &str) -> Option<&'a Scalar> {
    row.columns.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}
