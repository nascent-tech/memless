use super::invalid::invalid;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Statement;

pub(crate) fn one_statement(mut statements: Vec<Statement>) -> Result<Statement, QueryRefusal> {
    if statements.len() > 1 {
        return Err(outside("multiple statements"));
    }
    statements.pop().ok_or_else(|| invalid("no statement to run"))
}
