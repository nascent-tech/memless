use std::io;

use super::kind_label::kind_label;
use memless_domain::refusal::WriteRefusal::DiskWriteFailed;
use memless_domain::Refusal;

pub(crate) fn disk_failure(path: &str, error: io::Error) -> Refusal {
    Refusal::Write(DiskWriteFailed { path: path.to_string(), kind: kind_label(error.kind()) })
}
