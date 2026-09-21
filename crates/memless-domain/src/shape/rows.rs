use super::row::shape_row;
use crate::document::RawNode;
use crate::refusal::StructureRefusal;
use crate::refusal::StructureRefusal::TableNotRowList;
use crate::scalar::Scalar;

pub(crate) fn shape_rows(table: &str, value: &RawNode) -> Result<Vec<Vec<(String, Scalar)>>, StructureRefusal> {
    let RawNode::Sequence(elements) = value else {
        return Err(TableNotRowList { table: table.to_string() });
    };
    elements
        .iter()
        .enumerate()
        .map(|(index, element)| shape_row(table, index + 1, element))
        .collect()
}
