use super::check_where::check_where;
use super::edit_rows::edit_rows;
use super::table_index::table_index;
use super::targets::assignment_targets;
use super::unique_columns::check_unique;
use crate::base::table::Table;
use crate::query::Update;
use crate::refusal::Refusal;

pub(crate) fn update(tables: &[Table], spec: &Update) -> Result<(Vec<Table>, u64), Refusal> {
    check_unique(&spec.table, &assignment_targets(spec))?;
    let index = table_index(tables, &spec.table)?;
    check_where(&tables[index], &spec.filter)?;
    let mut next = tables.to_vec();
    let affected = edit_rows(&mut next[index], spec)?;
    Ok((next, affected))
}
