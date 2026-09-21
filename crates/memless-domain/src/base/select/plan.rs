use super::build_join::build_join;
use super::column_check::check_columns;
use super::find_joined::find_joined;
use super::join_plan::JoinPlan;
use super::resolve::find_table;
use crate::base::table::Table;
use crate::query::Select;
use crate::refusal::QueryRefusal;

pub(crate) struct Plan<'a> {
    pub from: &'a Table,
    pub join: Option<JoinPlan<'a>>,
}

pub(crate) fn plan<'a>(tables: &'a [Table], query: &Select) -> Result<Plan<'a>, QueryRefusal> {
    let from = find_table(tables, &query.from)?;
    let joined = find_joined(tables, query)?;
    check_columns(from, joined, query)?;
    let join = build_join(from, joined, query)?;
    Ok(Plan { from, join })
}
