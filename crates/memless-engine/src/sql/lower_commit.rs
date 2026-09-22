use super::reject_transaction_options::reject_transaction_options;
use memless_domain::query::Statement;
use memless_domain::QueryRefusal;
use sqlparser::ast::TransactionModifier;

pub(crate) fn lower_commit(
    chain: bool,
    end: bool,
    modifier: Option<TransactionModifier>,
) -> Result<Statement, QueryRefusal> {
    reject_transaction_options(chain || end || modifier.is_some())?;
    Ok(Statement::Commit)
}
