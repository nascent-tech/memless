use super::faults::file_empty;
use memless_domain::refusal::SourceRefusal;

pub(super) fn ensure_not_blank(path: &str, content: &str) -> Result<(), SourceRefusal> {
    if content.trim().is_empty() {
        return Err(file_empty(path));
    }
    Ok(())
}
