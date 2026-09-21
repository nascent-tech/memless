use std::fmt;

use super::Refusal;
use message::write_message;

mod message;

#[derive(Debug, Clone, PartialEq)]
pub enum WriteRefusal {
    ColumnCountMismatch { table: String, columns: usize, values: usize },
    ColumnRepeated { table: String, column: String },
    DiskWriteFailed { path: String, kind: String },
}

impl fmt::Display for WriteRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", write_message(self))
    }
}

impl From<WriteRefusal> for Refusal {
    fn from(refusal: WriteRefusal) -> Self {
        Refusal::Write(refusal)
    }
}
