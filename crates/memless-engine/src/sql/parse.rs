use super::analyse::analyse;
use super::lower_select::lower_select;
use super::one_statement::one_statement;
use super::query_of::query_of;
use super::reject_query_tail::reject_query_tail;
use super::select_of::select_of;
use memless_domain::query::Select;
use memless_domain::QueryRefusal;

pub fn parse(text: &str) -> Result<Select, QueryRefusal> {
    let statement = one_statement(analyse(text)?)?;
    let query = query_of(statement)?;
    reject_query_tail(&query)?;
    let select = select_of(query)?;
    lower_select(&select)
}
