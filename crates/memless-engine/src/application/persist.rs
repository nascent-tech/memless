use super::disk_failure::disk_failure;
use super::replace_file::ReplaceFile;
use crate::yaml::render;
use memless_domain::{Base, Refusal};

pub(crate) fn persist(replace: ReplaceFile, path: &str, base: &Base) -> Result<(), Refusal> {
    let text = render(&base.document());
    replace(path, &text).map_err(|error| disk_failure(path, error))
}
