use super::cell::cell;
use crate::base::row::Row;
use crate::document::RawNode;

pub(crate) fn row_node(row: &Row) -> RawNode {
    RawNode::Mapping(row.columns.iter().map(cell).collect())
}
