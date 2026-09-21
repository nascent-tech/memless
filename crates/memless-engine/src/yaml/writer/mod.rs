use std::io;

use cleanup::cleanup;
use residue::residue_path;
use swap::swap;
use write_temp::write_temp;

mod carry_permissions;
mod cleanup;
mod residue;
mod swap;
mod sync_parent;
mod write_temp;

pub fn replace_file(path: &str, text: &str) -> io::Result<()> {
    let residue = residue_path(path);
    if let Err(error) = write_temp(&residue, path, text) {
        cleanup(&residue);
        return Err(error);
    }
    if let Err(error) = swap(&residue, path) {
        cleanup(&residue);
        return Err(error);
    }
    Ok(())
}
