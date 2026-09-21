use std::io::ErrorKind;

pub(crate) fn kind_label(kind: ErrorKind) -> String {
    match kind {
        ErrorKind::PermissionDenied => "permission denied",
        ErrorKind::NotFound => "not found",
        ErrorKind::AlreadyExists => "already exists",
        _ => "other",
    }
    .to_string()
}
