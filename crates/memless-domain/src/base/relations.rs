use super::index::IdIndex;
use super::table::Table;
use super::table_relations::check_table_relations;
use crate::refusal::StructureRefusal;

pub(crate) fn check_relations(tables: &[Table], index: &IdIndex) -> Result<(), StructureRefusal> {
    tables.iter().try_for_each(|table| check_table_relations(index, table))
}
