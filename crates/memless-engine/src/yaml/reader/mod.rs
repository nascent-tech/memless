mod blank;
mod content;
mod decode;
mod faults;

use super::parse::parse;
use blank::ensure_not_blank;
use content::read_content;
use faults::invalid_yaml;
use memless_domain::document::RawDocument;
use memless_domain::refusal::SourceRefusal;

pub fn read(path: &str) -> Result<RawDocument, SourceRefusal> {
    let content = read_content(path)?;
    ensure_not_blank(path, &content)?;
    let root = parse(&content).map_err(|failure| invalid_yaml(path, failure.at))?;
    Ok(RawDocument { source: path.to_string(), root })
}
