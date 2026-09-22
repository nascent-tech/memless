use super::abandon::abandon;
use super::instance::Instance;
use super::open::open;
use super::replace_file::ReplaceFile;
use super::run_write::run_write;
use super::validate::validate;
use crate::sql::ParseSql;
use memless_domain::query::Statement;
use memless_domain::refusal::QueryRefusal::OutsideSubset;
use memless_domain::Refusal;

pub fn execute(parse: ParseSql, replace: ReplaceFile, instance: &mut Instance, text: &str) -> Result<u64, Refusal> {
    match parse(text)? {
        Statement::Begin => open(instance),
        Statement::Commit => validate(replace, instance),
        Statement::Rollback => abandon(instance),
        Statement::Write(statement) => run_write(replace, instance, &statement),
        Statement::Select(_) => Err(Refusal::Query(OutsideSubset { construct: "a SELECT in execute".to_string() })),
    }
}
