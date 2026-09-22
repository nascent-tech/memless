use super::instance::Instance;
use super::persist::persist;
use super::replace_file::ReplaceFile;
use memless_domain::query::Write;
use memless_domain::Refusal;

pub(crate) fn write_isolated(replace: ReplaceFile, instance: &mut Instance, statement: &Write) -> Result<u64, Refusal> {
    let applied = instance.base.write(statement)?;
    if applied.base == instance.base {
        return Ok(applied.affected);
    }
    persist(replace, &instance.path, &applied.base)?;
    instance.base = applied.base;
    Ok(applied.affected)
}
