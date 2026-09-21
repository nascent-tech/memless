use std::fs;
use std::io;
use std::path::Path;

use super::sync_parent::sync_parent;

pub(super) fn swap(residue: &Path, path: &str) -> io::Result<()> {
    fs::rename(residue, path)?;
    sync_parent(path)
}
