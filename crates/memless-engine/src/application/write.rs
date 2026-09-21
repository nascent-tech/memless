use super::apply::apply;
use super::instance::Instance;
use super::persist::persist;
use super::replace_file::ReplaceFile;
use crate::sql::ParseSql;
use memless_domain::query::Statement;
use memless_domain::refusal::QueryRefusal::OutsideSubset;
use memless_domain::Refusal;

pub fn write(parse: ParseSql, replace: ReplaceFile, instance: &mut Instance, text: &str) -> Result<u64, Refusal> {
    let Statement::Write(statement) = parse(text)? else {
        return Err(Refusal::Query(OutsideSubset { construct: "a SELECT in execute".to_string() }));
    };
    let applied = apply(&instance.base, &statement)?;
    if applied.base == instance.base {
        return Ok(applied.affected);
    }
    persist(replace, &instance.path, &applied.base)?;
    instance.base = applied.base;
    Ok(applied.affected)
}
