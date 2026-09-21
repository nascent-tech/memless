use super::candidate::Candidate;
use super::cell::cell;
use super::holder_label::holder_label;
use super::plan::Plan;
use crate::query::ColumnRef;
use crate::refusal::RowLabel;
use crate::scalar::Scalar;

pub(crate) fn present_cell<'a>(
    plan: &Plan<'a>,
    candidate: &Candidate<'a>,
    column: &ColumnRef,
) -> Option<(RowLabel, Scalar)> {
    let value = cell(plan, candidate, column)?;
    Some((holder_label(plan, candidate, column), value.clone()))
}
