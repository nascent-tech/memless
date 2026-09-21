use super::build_table::build_table;
use super::table::Table;
use crate::refusal::StructureRefusal;
use crate::shape::ShapedTable;

pub(crate) fn build_tables(shaped: &[ShapedTable]) -> Result<Vec<Table>, StructureRefusal> {
    shaped.iter().map(build_table).collect()
}
