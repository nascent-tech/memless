use super::analyse::analyse;
use super::lower_statement::lower_statement;
use super::one_statement::one_statement;
use memless_domain::query::Statement;
use memless_domain::QueryRefusal;

pub fn parse(text: &str) -> Result<Statement, QueryRefusal> {
    lower_statement(one_statement(analyse(text)?)?)
}
