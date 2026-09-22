use super::where_column::where_column;
use crate::base::filter::columns;
use crate::base::table::Table;
use crate::query::Filter;
use crate::refusal::Refusal;

pub(crate) fn check_where(table: &Table, filter: &Option<Filter>) -> Result<(), Refusal> {
    let Some(filter) = filter else {
        return Ok(());
    };
    columns(filter).into_iter().try_for_each(|column| where_column(table, column))
}
