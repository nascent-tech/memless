use super::table::Table;
use super::table_uniqueness::check_table_uniqueness;
use crate::refusal::StructureRefusal;

pub(crate) fn check_uniqueness(tables: &[Table]) -> Result<(), StructureRefusal> {
    tables.iter().try_for_each(check_table_uniqueness)
}
