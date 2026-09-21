use super::join_plan::JoinPlan;
use super::matched_plan::matched_plan;
use super::targets_id::targets_id;
use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn orientation<'a>(
    from: &'a Table,
    joined: &'a Table,
    relation: &ColumnRef,
    id: &ColumnRef,
) -> Option<JoinPlan<'a>> {
    let (other, from_carries) = super::join_holder::join_holder(from, joined, relation)?;
    if !targets_id(&relation.column, other, id) {
        return None;
    }
    Some(matched_plan(joined, from_carries, relation))
}
