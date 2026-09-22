use super::instance::Instance;
use memless_domain::refusal::TransactionRefusal::NoOpenTransaction;
use memless_domain::Refusal;

pub(crate) fn abandon(instance: &mut Instance) -> Result<u64, Refusal> {
    if instance.transaction.is_none() {
        return Err(Refusal::Transaction(NoOpenTransaction));
    }
    instance.transaction = None;
    Ok(0)
}
