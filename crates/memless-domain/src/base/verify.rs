use super::index::build_index;
use super::relations::check_relations;
use super::table::Table;
use super::uniqueness::check_uniqueness;
use crate::refusal::StructureRefusal;

pub(crate) fn verify(tables: &[Table]) -> Result<(), StructureRefusal> {
    check_uniqueness(tables)?;
    let index = build_index(tables);
    check_relations(tables, &index)
}
