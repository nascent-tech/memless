use super::reject_transaction_options::reject_transaction_options;
use memless_domain::query::Statement;
use memless_domain::QueryRefusal;
use sqlparser::ast::{TransactionMode, TransactionModifier};

pub(crate) fn lower_begin(
    modes: Vec<TransactionMode>,
    modifier: Option<TransactionModifier>,
) -> Result<Statement, QueryRefusal> {
    reject_transaction_options(!modes.is_empty() || modifier.is_some())?;
    Ok(Statement::Begin)
}
