use super::base_to_read::base_to_read;
use super::instance::Instance;
use crate::sql::ParseSql;
use memless_domain::query::Statement;
use memless_domain::refusal::QueryRefusal::OutsideSubset;
use memless_domain::{Refusal, Rows};

pub fn query(parse: ParseSql, instance: &Instance, text: &str) -> Result<Rows, Refusal> {
    let Statement::Select(select) = parse(text)? else {
        return Err(Refusal::Query(OutsideSubset { construct: "a non-SELECT in query".to_string() }));
    };
    Ok(base_to_read(instance).select(&select)?)
}
