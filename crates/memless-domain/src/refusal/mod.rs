use std::fmt;

mod invalid_yaml;
mod query;
mod row_label;
mod source;
mod structure;
mod text_position;
mod write;

pub use query::QueryRefusal;
pub use row_label::RowLabel;
pub use source::SourceRefusal;
pub use structure::StructureRefusal;
pub use text_position::TextPosition;
pub use write::WriteRefusal;

#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    Source(SourceRefusal),
    Structure(StructureRefusal),
    Query(QueryRefusal),
    Write(WriteRefusal),
}

impl fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Source(refusal) => write!(formatter, "{refusal}"),
            Refusal::Structure(refusal) => write!(formatter, "{refusal}"),
            Refusal::Query(refusal) => write!(formatter, "{refusal}"),
            Refusal::Write(refusal) => write!(formatter, "{refusal}"),
        }
    }
}

impl From<QueryRefusal> for Refusal {
    fn from(refusal: QueryRefusal) -> Self {
        Refusal::Query(refusal)
    }
}

impl std::error::Error for Refusal {}
