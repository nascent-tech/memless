use super::col::Col;
use super::plan::Plan;
use super::ref_col::ref_col;
use crate::query::ColumnRef;

pub(crate) fn named_cols(plan: &Plan, columns: &[ColumnRef]) -> Vec<Col> {
    columns.iter().map(|column| ref_col(plan, column)).collect()
}
