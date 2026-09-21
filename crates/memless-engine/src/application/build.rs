use memless_domain::document::RawDocument;
use memless_domain::{Base, Refusal};

pub(super) fn build(document: RawDocument) -> Result<Base, Refusal> {
    Ok(Base::load(document)?)
}
