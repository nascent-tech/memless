use super::check_one_column::check_one_column;
use super::referenced_columns::referenced_columns;
use crate::base::table::Table;
use crate::query::Select;
use crate::refusal::QueryRefusal;

pub(crate) fn check_columns(from: &Table, joined: Option<&Table>, query: &Select) -> Result<(), QueryRefusal> {
    referenced_columns(query)
        .iter()
        .try_for_each(|column| check_one_column(from, joined, column))
}
