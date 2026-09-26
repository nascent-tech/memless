use super::plan::Plan;
use super::target_table::target_table;
use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn holder_table<'a>(plan: &Plan<'a>, column: &ColumnRef) -> &'a Table {
    target_table(plan.from, plan.join.as_ref().map(|join| join.table), column)
}
