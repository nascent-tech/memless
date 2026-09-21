use crate::document::RawDocument;
use crate::refusal::StructureRefusal;
use crate::shape::check_shape;

mod build;
mod build_row;
mod build_table;
mod column_relation;
mod duplicates;
mod faults;
mod index;
mod relation_context;
mod relation_lookup;
mod relation_resolve;
mod relations;
mod row;
mod row_id;
mod row_relations;
mod table;
mod table_relations;
mod table_uniqueness;
mod uniqueness;

use build::build_tables;
use index::build_index;
use relations::check_relations;
use table::Table;
use uniqueness::check_uniqueness;

pub struct Base {
    #[allow(dead_code)]
    tables: Vec<Table>,
}

impl Base {
    pub fn load(document: RawDocument) -> Result<Base, StructureRefusal> {
        let shaped = check_shape(document)?;
        let tables = build_tables(&shaped)?;
        check_uniqueness(&tables)?;
        let index = build_index(&tables);
        check_relations(&tables, &index)?;
        Ok(Base { tables })
    }
}
