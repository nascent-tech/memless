use super::rows::shape_rows;
use super::tables::{ensure_new_table, table_name};
use super::ShapedTable;
use crate::document::{RawKey, RawNode};
use crate::refusal::StructureRefusal;

pub(crate) fn shape_table(
    seen: &mut Vec<String>,
    position: usize,
    key: &RawKey,
    value: &RawNode,
) -> Result<ShapedTable, StructureRefusal> {
    let name = table_name(key, position)?;
    ensure_new_table(seen, &name)?;
    let rows = shape_rows(&name, value)?;
    Ok(ShapedTable { name, rows })
}
