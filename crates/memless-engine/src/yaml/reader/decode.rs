use super::faults::invalid_yaml;
use memless_domain::refusal::SourceRefusal;

pub(super) fn reject_bom(path: &str, text: String) -> Result<String, SourceRefusal> {
    if text.starts_with('\u{FEFF}') {
        return Err(invalid_yaml(path, None));
    }
    Ok(text)
}
