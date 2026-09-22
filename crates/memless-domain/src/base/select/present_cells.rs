use super::candidate::Candidate;
use super::plan::Plan;
use super::present_cell::present_cell;
use crate::query::ColumnRef;
use crate::refusal::RowLabel;
use crate::scalar::Scalar;

pub(crate) fn present_cells<'a>(
    plan: &Plan<'a>,
    column: &ColumnRef,
    candidates: &[Candidate<'a>],
) -> Vec<(RowLabel, Scalar)> {
    candidates
        .iter()
        .filter_map(|candidate| present_cell(plan, candidate, column))
        .collect()
}
