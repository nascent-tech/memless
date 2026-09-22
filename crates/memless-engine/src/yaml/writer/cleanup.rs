use std::fs;
use std::path::Path;

pub(super) fn cleanup(residue: &Path) {
    let _ = fs::remove_file(residue);
}
