use super::plan::Plan;
use crate::query::ColumnRef;

pub(crate) fn is_joined_ref(plan: &Plan, column: &ColumnRef) -> bool {
    match (&column.table, &plan.join) {
        (Some(name), Some(join)) => *name == join.table.name,
        _ => false,
    }
}
