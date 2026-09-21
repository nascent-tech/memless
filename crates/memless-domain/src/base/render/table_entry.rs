use super::row_node::row_node;
use crate::base::table::Table;
use crate::document::{RawKey, RawNode};

pub(crate) fn table_entry(table: &Table) -> (RawKey, RawNode) {
    let rows = table.rows.iter().map(row_node).collect();
    (RawKey::Text(table.name.clone()), RawNode::Sequence(rows))
}
