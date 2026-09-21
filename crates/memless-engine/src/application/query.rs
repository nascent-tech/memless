use crate::sql::ParseSql;
use memless_domain::{Base, Refusal, Rows};

pub fn query(parse: ParseSql, base: &Base, text: &str) -> Result<Rows, Refusal> {
    let select = parse(text)?;
    let rows = base.select(&select)?;
    Ok(rows)
}
