use std::path::{Path, PathBuf};

pub(super) fn residue_path(path: &str) -> PathBuf {
    let original = Path::new(path);
    let name = original.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    original.with_file_name(format!(".{name}.memless-tmp"))
}
