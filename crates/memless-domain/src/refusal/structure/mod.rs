use std::fmt;

use super::{Refusal, RowLabel};
use crate::scalar::Scalar;
use crate::Id;

mod coherence_kind;
mod coherence_link;
mod field_shape;
mod non_text_key;
mod table_shape;

use coherence_kind::coherence_kind_message;
use coherence_link::coherence_link_message;
use field_shape::field_shape_message;
use non_text_key::non_text_key_message;
use table_shape::table_shape_message;

#[derive(Debug, Clone, PartialEq)]
pub enum StructureRefusal {
    NoTableDeclared { source: String },
    TableNotRowList { table: String },
    RowNotFieldSet { table: String, position: usize },
    NestedValue { table: String, row: RowLabel, column: String },
    DuplicateTableKey { table: String },
    DuplicateColumnKey { table: String, row: RowLabel, column: String },
    NonTextKey { rendered: String, table: Option<String>, position: usize },
    MissingId { table: String, position: usize },
    IdNotTextOrInteger { table: String, position: usize, value: Scalar },
    DuplicateId { table: String, id: Id, positions: (usize, usize) },
    BrokenRelation { table: String, row: RowLabel, column: String, target_table: String, value: Scalar },
}

impl fmt::Display for StructureRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = table_shape_message(self)
            .or_else(|| field_shape_message(self))
            .or_else(|| non_text_key_message(self))
            .or_else(|| coherence_kind_message(self))
            .or_else(|| coherence_link_message(self));
        write!(formatter, "{}", message.unwrap_or_default())
    }
}

impl From<StructureRefusal> for Refusal {
    fn from(refusal: StructureRefusal) -> Self {
        Refusal::Structure(refusal)
    }
}
