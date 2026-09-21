use std::fmt;

use crate::Id;

#[derive(Debug, Clone, PartialEq)]
pub enum RowLabel {
    Id(Id),
    Position(usize),
}

impl fmt::Display for RowLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowLabel::Id(id) => write!(formatter, "{id}"),
            RowLabel::Position(position) => write!(formatter, "{position}"),
        }
    }
}
