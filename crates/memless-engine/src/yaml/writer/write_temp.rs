use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::carry_permissions::carry_permissions;

pub(super) fn write_temp(residue: &Path, path: &str, text: &str) -> io::Result<()> {
    let mut file = File::create(residue)?;
    file.write_all(text.as_bytes())?;
    file.sync_all()?;
    carry_permissions(path, residue)
}
