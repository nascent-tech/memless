use super::join_on::join_on;
use super::lower_on::lower_on;
use super::outside::outside;
use super::table_name::table_name;
use memless_domain::query::Join;
use memless_domain::QueryRefusal;
use sqlparser::ast::Join as SqlJoin;

pub(crate) fn one_join(join: &SqlJoin, from_name: &str) -> Result<Join, QueryRefusal> {
    let table = table_name(&join.relation)?;
    if table == from_name {
        return Err(outside("self join"));
    }
    let condition = join_on(&join.join_operator)?;
    let (left, right) = lower_on(condition)?;
    Ok(Join { table, left, right })
}
