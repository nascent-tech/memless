use crate::document::RawDocument;
use crate::refusal::StructureRefusal;

use field_site::ShapedRow;

mod field_folding;
mod field_helpers;
mod field_parts;
mod field_site;
mod fields;
mod row;
mod row_folding;
mod rows;
mod table;
mod tables;

use table::shape_table;
use tables::table_entries;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ShapedTable {
    pub name: String,
    pub rows: Vec<ShapedRow>,
}

pub(crate) fn check_shape(document: RawDocument) -> Result<Vec<ShapedTable>, StructureRefusal> {
    let entries = table_entries(&document)?;
    let mut seen = Vec::with_capacity(entries.len());
    entries
        .iter()
        .enumerate()
        .map(|(index, (key, value))| shape_table(&mut seen, index + 1, key, value))
        .collect()
}
