use std::fs;
use std::io;
use std::path::Path;

pub(super) fn carry_permissions(path: &str, residue: &Path) -> io::Result<()> {
    let Ok(metadata) = fs::metadata(path) else {
        return Ok(());
    };
    fs::set_permissions(residue, metadata.permissions())
}
