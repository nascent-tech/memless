use super::StructureRefusal;
use super::StructureRefusal::{IdNotTextOrInteger, MissingId};

pub(crate) fn coherence_kind_message(refusal: &StructureRefusal) -> Option<String> {
    match refusal {
        MissingId { table, position } => Some(format!("row {position} in {table:?} has no id")),
        IdNotTextOrInteger { table, position, value } => Some(format!(
            "row {position} in {table:?} has a non-text non-integer id: {value}"
        )),
        _ => None,
    }
}
