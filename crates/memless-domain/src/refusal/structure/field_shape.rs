use super::StructureRefusal;
use super::StructureRefusal::{DuplicateColumnKey, NestedValue};

pub(crate) fn field_shape_message(refusal: &StructureRefusal) -> Option<String> {
    match refusal {
        NestedValue { table, row, column } => Some(format!("nested value in {column:?} of row {row} in {table:?}")),
        DuplicateColumnKey { table, row, column } => Some(format!("{column:?} duplicated in row {row} of {table:?}")),
        _ => None,
    }
}
