use super::instance::Instance;
use super::reopen::reopen;
use super::replace_file::ReplaceFile;
use super::write_isolated::write_isolated;
use memless_domain::query::Write;
use memless_domain::Refusal;

pub(crate) fn run_write(replace: ReplaceFile, instance: &mut Instance, statement: &Write) -> Result<u64, Refusal> {
    let Some(working) = instance.transaction.take() else {
        return write_isolated(replace, instance, statement);
    };
    let applied = match working.apply_write(statement) {
        Ok(applied) => applied,
        Err(refusal) => return reopen(instance, working, refusal),
    };
    instance.transaction = Some(applied.base);
    Ok(applied.affected)
}
