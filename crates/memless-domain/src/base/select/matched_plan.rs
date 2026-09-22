use super::join_plan::JoinPlan;
use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn matched_plan<'a>(joined: &'a Table, from_carries: bool, relation: &ColumnRef) -> JoinPlan<'a> {
    JoinPlan { table: joined, from_carries, column: relation.column.clone() }
}
