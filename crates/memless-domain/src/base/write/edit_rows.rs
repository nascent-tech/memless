use super::update_one::update_one;
use crate::base::table::Table;
use crate::query::Update;
use crate::refusal::Refusal;

pub(crate) fn edit_rows(table: &mut Table, spec: &Update) -> Result<u64, Refusal> {
    let mut affected = 0;
    for position in 0..table.rows.len() {
        affected += update_one(table, position, spec)?;
    }
    Ok(affected)
}
