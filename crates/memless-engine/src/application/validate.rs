use super::instance::Instance;
use super::persist::persist;
use super::replace_file::ReplaceFile;
use memless_domain::refusal::TransactionRefusal::NoOpenTransaction;
use memless_domain::Refusal;

pub(crate) fn validate(replace: ReplaceFile, instance: &mut Instance) -> Result<u64, Refusal> {
    let Some(working) = instance.transaction.take() else {
        return Err(Refusal::Transaction(NoOpenTransaction));
    };
    working.verify_state()?;
    if working == instance.base {
        return Ok(0);
    }
    persist(replace, &instance.path, &working)?;
    instance.base = working;
    Ok(0)
}
