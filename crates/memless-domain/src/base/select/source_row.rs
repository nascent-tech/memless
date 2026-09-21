use super::candidate::Candidate;
use super::plan::Plan;
use crate::base::row::Row;
use crate::query::ColumnRef;

pub(crate) fn source_row<'a>(plan: &Plan<'a>, candidate: &Candidate<'a>, column: &ColumnRef) -> Option<&'a Row> {
    match (&column.table, &plan.join) {
        (Some(name), Some(join)) if *name == join.table.name => candidate.joined,
        _ => Some(candidate.from),
    }
}
