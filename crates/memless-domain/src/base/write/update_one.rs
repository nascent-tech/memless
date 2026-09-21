use super::matches_row::matches_row;
use super::rebuild::rebuild_row;
use crate::base::table::Table;
use crate::query::Update;
use crate::refusal::Refusal;

pub(crate) fn update_one(table: &mut Table, position: usize, spec: &Update) -> Result<u64, Refusal> {
    if !matches_row(&table.rows[position], &spec.filter) {
        return Ok(0);
    }
    let row = rebuild_row(&table.name, position, &table.rows[position], &spec.assignments)?;
    table.rows[position] = row;
    Ok(1)
}
