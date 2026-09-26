use super::build::build;
use super::instance::Instance;
use super::load::ReadSource;
use memless_domain::refusal::TransactionRefusal::OpenDuringReload;
use memless_domain::Refusal;

pub fn reload(read: ReadSource, instance: &mut Instance) -> Result<(), Refusal> {
    if instance.transaction.is_some() {
        return Err(Refusal::Transaction(OpenDuringReload));
    }
    let document = read(&instance.path)?;
    instance.base = build(document)?;
    Ok(())
}
