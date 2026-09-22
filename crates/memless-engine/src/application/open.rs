use super::instance::Instance;
use memless_domain::refusal::TransactionRefusal::AlreadyOpen;
use memless_domain::Refusal;

pub(crate) fn open(instance: &mut Instance) -> Result<u64, Refusal> {
    if instance.transaction.is_some() {
        return Err(Refusal::Transaction(AlreadyOpen));
    }
    instance.transaction = Some(instance.base.clone());
    Ok(0)
}
