use std::ptr;

use super::candidate::Candidate;
use super::holder_rows::holder_rows;
use super::holder_table::holder_table;
use super::plan::Plan;
use super::row_column::row_column;
use crate::base::row::Row;
use crate::query::ColumnRef;
use crate::scalar::Scalar;

pub(crate) fn held_cells<'a>(
    plan: &Plan<'a>,
    candidates: &[Candidate<'a>],
    column: &ColumnRef,
) -> Vec<(&'a Row, &'a Scalar)> {
    let holders = holder_rows(plan, candidates, column);
    let rows = holder_table(plan, column).rows.iter().filter(|row| holders.contains(&ptr::from_ref(*row)));
    rows.filter_map(|row| row_column(row, &column.column).map(|value| (row, value))).collect()
}
