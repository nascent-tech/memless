use super::col_header::col_header;
use super::is_joined_ref::is_joined_ref;
use super::plan::Plan;
use super::ref_table_name::ref_table_name;
use crate::query::ColumnRef;

pub(crate) fn header_name(plan: &Plan, column: &ColumnRef) -> String {
    let joined = is_joined_ref(plan, column);
    col_header(&ref_table_name(plan, joined), &column.column, plan.join.is_some())
}
