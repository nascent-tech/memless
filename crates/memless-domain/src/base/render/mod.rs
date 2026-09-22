use super::Base;
use crate::document::{RawDocument, RawNode};
use table_entry::table_entry;

mod cell;
mod explicit;
mod raw_scalar;
mod row_node;
mod table_entry;

impl Base {
    pub fn document(&self) -> RawDocument {
        let entries = self.tables.iter().map(table_entry).collect();
        RawDocument { source: String::new(), root: RawNode::Mapping(entries) }
    }
}
