use crate::document::RawDocument;
use crate::refusal::StructureRefusal;
use crate::shape::check_shape;

mod build;
mod build_row;
mod build_table;
mod column_relation;
mod duplicates;
mod faults;
mod filter;
mod index;
mod relation_context;
mod relation_lookup;
mod relation_resolve;
mod relations;
mod render;
mod row;
mod row_id;
mod row_relations;
mod select;
mod table;
mod table_relations;
mod table_uniqueness;
mod uniqueness;
mod verify;
mod verify_state;
mod write;

use build::build_tables;
use table::Table;
use verify::verify;

pub use write::Applied;

#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    tables: Vec<Table>,
}

impl Base {
    pub fn load(document: RawDocument) -> Result<Base, StructureRefusal> {
        let shaped = check_shape(document)?;
        let tables = build_tables(&shaped)?;
        verify(&tables)?;
        Ok(Base { tables })
    }
}
