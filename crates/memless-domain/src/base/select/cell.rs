use super::candidate::Candidate;
use super::plan::Plan;
use super::row_column::row_column;
use super::source_row::source_row;
use crate::query::ColumnRef;
use crate::scalar::Scalar;

pub(crate) fn cell<'a>(plan: &Plan<'a>, candidate: &Candidate<'a>, column: &ColumnRef) -> Option<&'a Scalar> {
    let row = source_row(plan, candidate, column)?;
    row_column(row, &column.column)
}
