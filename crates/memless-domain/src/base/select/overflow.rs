use super::header_table::header_table;
use super::plan::Plan;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::SumOverflow;

pub(crate) fn overflow(plan: &Plan, column: &ColumnRef) -> QueryRefusal {
    let table = header_table(plan, column);
    SumOverflow { table, column: column.column.clone() }
}
