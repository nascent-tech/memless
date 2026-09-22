use super::reject_transaction_options::reject_transaction_options;
use memless_domain::query::Statement;
use memless_domain::QueryRefusal;
use sqlparser::ast::Ident;

pub(crate) fn lower_rollback(chain: bool, savepoint: Option<Ident>) -> Result<Statement, QueryRefusal> {
    reject_transaction_options(chain || savepoint.is_some())?;
    Ok(Statement::Rollback)
}
