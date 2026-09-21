use std::fmt;

use super::invalid_yaml::invalid_yaml_message;
use super::{Refusal, TextPosition};

#[derive(Debug, Clone, PartialEq)]
pub enum SourceRefusal {
    PathHasNoFile { path: String },
    FileNotReadable { path: String },
    FileEmpty { path: String },
    InvalidYaml { path: String, at: Option<TextPosition> },
}

impl fmt::Display for SourceRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceRefusal::PathHasNoFile { path } => write!(formatter, "no file at path {path:?}"),
            SourceRefusal::FileNotReadable { path } => write!(formatter, "file at path {path:?} is not readable"),
            SourceRefusal::FileEmpty { path } => write!(formatter, "file at path {path:?} is empty"),
            SourceRefusal::InvalidYaml { path, at } => write!(formatter, "{}", invalid_yaml_message(path, at)),
        }
    }
}

impl From<SourceRefusal> for Refusal {
    fn from(refusal: SourceRefusal) -> Self {
        Refusal::Source(refusal)
    }
}
