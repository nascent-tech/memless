use super::index::IdIndex;
use super::row_relations::check_row_relations;
use super::table::Table;
use crate::refusal::StructureRefusal;

pub(crate) fn check_table_relations(index: &IdIndex, table: &Table) -> Result<(), StructureRefusal> {
    table.rows.iter().try_for_each(|row| check_row_relations(index, table, row))
}
