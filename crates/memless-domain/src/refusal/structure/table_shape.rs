use super::StructureRefusal;
use super::StructureRefusal::{DuplicateTableKey, NoTableDeclared, RowNotFieldSet, TableNotRowList};

pub(crate) fn table_shape_message(refusal: &StructureRefusal) -> Option<String> {
    match refusal {
        NoTableDeclared { source } => Some(format!("no table declared in {source:?}")),
        TableNotRowList { table } => Some(format!("table {table:?} is not a list of rows")),
        RowNotFieldSet { table, position } => Some(format!("row {position} in {table:?} is not a field set")),
        DuplicateTableKey { table } => Some(format!("duplicate table key {table:?}")),
        _ => None,
    }
}
