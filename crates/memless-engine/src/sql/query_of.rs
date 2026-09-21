use super::outside::outside;
use super::statement_label::statement_label;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Query, Statement};

pub(crate) fn query_of(statement: Statement) -> Result<Query, QueryRefusal> {
    match statement {
        Statement::Query(query) => Ok(*query),
        other => Err(outside(statement_label(&other))),
    }
}
