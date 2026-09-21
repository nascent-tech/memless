use std::fmt;

mod invalid_yaml;
mod row_label;
mod source;
mod structure;
mod text_position;

pub use row_label::RowLabel;
pub use source::SourceRefusal;
pub use structure::StructureRefusal;
pub use text_position::TextPosition;

#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    Source(SourceRefusal),
    Structure(StructureRefusal),
}

impl fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Source(refusal) => write!(formatter, "{refusal}"),
            Refusal::Structure(refusal) => write!(formatter, "{refusal}"),
        }
    }
}

impl std::error::Error for Refusal {}
