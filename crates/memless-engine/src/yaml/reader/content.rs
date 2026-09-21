use std::fs;
use std::path::Path;

use super::decode::reject_bom;
use super::faults::{file_not_readable, invalid_yaml, path_has_no_file};
use memless_domain::refusal::SourceRefusal;

pub(super) fn read_content(path: &str) -> Result<String, SourceRefusal> {
    if !Path::new(path).is_file() {
        return Err(path_has_no_file(path));
    }
    let bytes = fs::read(path).map_err(|_| file_not_readable(path))?;
    let text = String::from_utf8(bytes).map_err(|_| invalid_yaml(path, None))?;
    reject_bom(path, text)
}
