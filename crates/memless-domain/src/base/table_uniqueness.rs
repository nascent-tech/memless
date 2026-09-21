use super::duplicates::first_duplicate;
use super::faults::duplicate_id;
use super::table::Table;
use crate::refusal::StructureRefusal;

pub(crate) fn check_table_uniqueness(table: &Table) -> Result<(), StructureRefusal> {
    let Some((first, second)) = first_duplicate(&table.rows) else {
        return Ok(());
    };
    let id = table.rows[second - 1].id.clone();
    Err(duplicate_id(&table.name, id, first, second))
}
