use super::candidate::Candidate;
use super::col::Col;
use super::col_row::col_row;
use super::row_column::row_column;
use crate::scalar::Scalar;

pub(crate) fn col_value<'a>(candidate: &Candidate<'a>, col: &Col) -> Option<Scalar> {
    let row = col_row(candidate, col)?;
    row_column(row, &col.name).cloned()
}
