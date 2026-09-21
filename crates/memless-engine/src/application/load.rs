use super::build::build;
use memless_domain::document::RawDocument;
use memless_domain::refusal::SourceRefusal;
use memless_domain::{Base, Refusal};

pub type ReadSource = fn(&str) -> Result<RawDocument, SourceRefusal>;

pub fn load(read: ReadSource, path: &str) -> Result<Base, Refusal> {
    let document = read(path)?;
    build(document)
}
