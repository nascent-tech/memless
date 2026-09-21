use super::candidate::Candidate;
use super::cell::cell;
use super::plan::Plan;
use crate::query::ColumnRef;

pub(crate) fn count_present<'a>(plan: &Plan<'a>, column: &ColumnRef, candidates: &[Candidate<'a>]) -> i64 {
    candidates.iter().filter(|candidate| cell(plan, candidate, column).is_some()).count() as i64
}
