use super::candidate::Candidate;
use super::plan::Plan;
use super::source_row::source_row;
use crate::query::ColumnRef;
use crate::refusal::RowLabel;

pub(crate) fn holder_label<'a>(plan: &Plan<'a>, candidate: &Candidate<'a>, column: &ColumnRef) -> RowLabel {
    match source_row(plan, candidate, column) {
        Some(row) => RowLabel::Id(row.id.clone()),
        None => RowLabel::Position(0),
    }
}
