use super::build::build;
use super::instance::Instance;
use memless_domain::document::RawDocument;
use memless_domain::refusal::SourceRefusal;
use memless_domain::Refusal;

pub type ReadSource = fn(&str) -> Result<RawDocument, SourceRefusal>;

pub fn load(read: ReadSource, path: &str) -> Result<Instance, Refusal> {
    let document = read(path)?;
    let base = build(document)?;
    Ok(Instance { path: path.to_string(), base })
}
