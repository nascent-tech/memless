use super::fields::shape_fields;
use crate::document::RawNode;
use crate::refusal::StructureRefusal;
use crate::refusal::StructureRefusal::RowNotFieldSet;
use crate::scalar::Scalar;

pub(crate) fn shape_row(
    table: &str,
    position: usize,
    element: &RawNode,
) -> Result<Vec<(String, Scalar)>, StructureRefusal> {
    let RawNode::Mapping(entries) = element else {
        return Err(RowNotFieldSet { table: table.to_string(), position });
    };
    shape_fields(table, position, entries)
}
