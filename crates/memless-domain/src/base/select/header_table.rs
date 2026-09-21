use super::is_joined_ref::is_joined_ref;
use super::plan::Plan;
use super::ref_table_name::ref_table_name;
use crate::query::ColumnRef;

pub(crate) fn header_table(plan: &Plan, column: &ColumnRef) -> String {
    ref_table_name(plan, is_joined_ref(plan, column))
}
