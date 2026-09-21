use super::join_plan::{join_plan, JoinPlan};
use crate::base::table::Table;
use crate::query::Select;
use crate::refusal::QueryRefusal;

pub(crate) fn build_join<'a>(
    from: &'a Table,
    joined: Option<&'a Table>,
    query: &Select,
) -> Result<Option<JoinPlan<'a>>, QueryRefusal> {
    match (&query.join, joined) {
        (Some(join), Some(table)) => join_plan(from, table, join).map(Some),
        _ => Ok(None),
    }
}
