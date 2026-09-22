use super::join_refusal::join_refusal;
use super::orientation::orientation;
use crate::base::table::Table;
use crate::query::Join;
use crate::refusal::QueryRefusal;

pub(crate) struct JoinPlan<'a> {
    pub table: &'a Table,
    pub from_carries: bool,
    pub column: String,
}

pub(crate) fn join_plan<'a>(from: &'a Table, joined: &'a Table, join: &Join) -> Result<JoinPlan<'a>, QueryRefusal> {
    if let Some(plan) = orientation(from, joined, &join.left, &join.right) {
        return Ok(plan);
    }
    if let Some(plan) = orientation(from, joined, &join.right, &join.left) {
        return Ok(plan);
    }
    Err(join_refusal(from, joined, join))
}
