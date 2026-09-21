use super::build_row::build_row;
use super::row::Row;
use super::table::Table;
use crate::refusal::StructureRefusal;
use crate::shape::ShapedTable;

pub(crate) fn build_table(shaped: &ShapedTable) -> Result<Table, StructureRefusal> {
    let rows = shaped
        .rows
        .iter()
        .enumerate()
        .map(|(index, fields)| build_row(&shaped.name, index + 1, fields))
        .collect::<Result<Vec<Row>, _>>()?;
    Ok(Table { name: shaped.name.clone(), rows })
}
