use super::header_table::header_table;
use super::plan::Plan;
use crate::query::ColumnRef;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::SumNotNumber;
use crate::refusal::RowLabel;

pub(crate) fn not_number(plan: &Plan, column: &ColumnRef, row: &RowLabel) -> QueryRefusal {
    let table = header_table(plan, column);
    SumNotNumber { table, column: column.column.clone(), row: row.clone() }
}
