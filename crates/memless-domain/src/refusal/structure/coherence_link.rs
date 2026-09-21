use super::StructureRefusal;
use super::StructureRefusal::{BrokenRelation, DuplicateId};

pub(crate) fn coherence_link_message(refusal: &StructureRefusal) -> Option<String> {
    match refusal {
        DuplicateId { table, id, positions } => Some(format!(
            "duplicate id {id} in {table:?} at rows {} and {}",
            positions.0, positions.1
        )),
        BrokenRelation { table, row, column, target_table, value } => Some(format!(
            "broken relation {column:?} of row {row} in {table:?}: no row {value} in {target_table:?}"
        )),
        _ => None,
    }
}
