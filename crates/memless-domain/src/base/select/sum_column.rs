use super::candidate::Candidate;
use super::dispatch_sum::dispatch_sum;
use super::plan::Plan;
use super::present_cells::present_cells;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::scalar::Scalar;

pub(crate) fn sum_column<'a>(
    plan: &Plan<'a>,
    column: &ColumnRef,
    candidates: &[Candidate<'a>],
) -> Result<Option<Scalar>, QueryRefusal> {
    let present = present_cells(plan, column, candidates);
    let Some((_, first)) = present.first() else {
        return Ok(None);
    };
    dispatch_sum(plan, column, first, &present)
}
