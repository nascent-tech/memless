use std::collections::HashSet;
use std::ptr;

use super::candidate::Candidate;
use super::plan::Plan;
use super::source_row::source_row;
use crate::base::row::Row;
use crate::query::ColumnRef;

pub(crate) fn holder_rows<'a>(
    plan: &Plan<'a>,
    candidates: &[Candidate<'a>],
    column: &ColumnRef,
) -> HashSet<*const Row> {
    candidates
        .iter()
        .filter_map(|candidate| source_row(plan, candidate, column))
        .map(ptr::from_ref)
        .collect()
}
