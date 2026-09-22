use super::dialect::MemlessDialect;
use super::invalid::invalid;
use super::sql_detail::sql_detail;
use memless_domain::QueryRefusal;
use sqlparser::ast::Statement;
use sqlparser::parser::Parser;

pub(crate) fn analyse(text: &str) -> Result<Vec<Statement>, QueryRefusal> {
    Parser::parse_sql(&MemlessDialect, text).map_err(|error| invalid(&sql_detail(error)))
}
