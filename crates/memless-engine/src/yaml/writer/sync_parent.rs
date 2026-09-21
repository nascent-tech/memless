use std::fs::File;
use std::io;
use std::path::Path;

pub(super) fn sync_parent(path: &str) -> io::Result<()> {
    let Some(parent) = Path::new(path).parent() else {
        return Ok(());
    };
    let _ = File::open(parent).and_then(|directory| directory.sync_all());
    Ok(())
}
