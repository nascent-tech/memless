use crate::sql::ParseSql;
use memless_domain::query::Statement;
use memless_domain::refusal::QueryRefusal::OutsideSubset;
use memless_domain::{Base, Refusal, Rows};

pub fn query(parse: ParseSql, base: &Base, text: &str) -> Result<Rows, Refusal> {
    let Statement::Select(select) = parse(text)? else {
        return Err(Refusal::Query(OutsideSubset { construct: "a write in query".to_string() }));
    };
    Ok(base.select(&select)?)
}
