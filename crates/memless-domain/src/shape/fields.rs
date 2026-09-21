use super::field_parts::row_label;
use super::field_site::{FieldSite, ShapedRow};
use super::row_folding::fold_row;
use crate::document::{RawKey, RawNode};
use crate::refusal::StructureRefusal;

pub(crate) fn shape_fields(
    table: &str,
    position: usize,
    entries: &[(RawKey, RawNode)],
) -> Result<ShapedRow, StructureRefusal> {
    let label = row_label(entries, position);
    let site = FieldSite { table, position, label };
    fold_row(&site, entries)
}
