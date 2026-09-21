use memless_domain::refusal::SourceRefusal;
use memless_domain::refusal::SourceRefusal::{FileEmpty, FileNotReadable, InvalidYaml, PathHasNoFile};
use memless_domain::refusal::TextPosition;

pub(super) fn path_has_no_file(path: &str) -> SourceRefusal {
    PathHasNoFile { path: path.to_string() }
}

pub(super) fn file_not_readable(path: &str) -> SourceRefusal {
    FileNotReadable { path: path.to_string() }
}

pub(super) fn file_empty(path: &str) -> SourceRefusal {
    FileEmpty { path: path.to_string() }
}

pub(super) fn invalid_yaml(path: &str, at: Option<TextPosition>) -> SourceRefusal {
    InvalidYaml { path: path.to_string(), at }
}
