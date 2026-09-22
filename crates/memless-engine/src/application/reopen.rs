use super::instance::Instance;
use memless_domain::{Base, Refusal};

pub(crate) fn reopen(instance: &mut Instance, working: Base, refusal: Refusal) -> Result<u64, Refusal> {
    instance.transaction = Some(working);
    Err(refusal)
}
