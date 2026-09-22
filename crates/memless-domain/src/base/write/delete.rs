use super::check_where::check_where;
use super::cut_rows::cut_rows;
use super::table_index::table_index;
use crate::base::table::Table;
use crate::query::Delete;
use crate::refusal::Refusal;

pub(crate) fn delete(tables: &[Table], spec: &Delete) -> Result<(Vec<Table>, u64), Refusal> {
    let index = table_index(tables, &spec.table)?;
    check_where(&tables[index], &spec.filter)?;
    let mut next = tables.to_vec();
    let affected = cut_rows(&mut next[index], spec);
    Ok((next, affected))
}
