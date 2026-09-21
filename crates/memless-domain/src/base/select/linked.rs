use super::join_plan::JoinPlan;
use super::references::references;
use crate::base::row::Row;

pub(crate) fn linked(from: &Row, joined: &Row, join: &JoinPlan) -> bool {
    match join.from_carries {
        true => references(from, &join.column, &joined.id),
        false => references(joined, &join.column, &from.id),
    }
}
