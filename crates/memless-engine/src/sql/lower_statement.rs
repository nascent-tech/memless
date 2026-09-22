use super::lower_begin::lower_begin;
use super::lower_commit::lower_commit;
use super::lower_delete::lower_delete;
use super::lower_insert::lower_insert;
use super::lower_query::lower_query;
use super::lower_rollback::lower_rollback;
use super::lower_update::lower_update;
use super::outside::outside;
use super::statement_label::statement_label;
use memless_domain::query::Statement;
use memless_domain::QueryRefusal;
use sqlparser::ast::Statement as SqlStatement;

pub(crate) fn lower_statement(statement: SqlStatement) -> Result<Statement, QueryRefusal> {
    match statement {
        SqlStatement::Query(query) => lower_query(query).map(Statement::Select),
        SqlStatement::Insert(insert) => lower_insert(insert).map(Statement::Write),
        update @ SqlStatement::Update { .. } => lower_update(update).map(Statement::Write),
        SqlStatement::Delete(delete) => lower_delete(delete).map(Statement::Write),
        SqlStatement::StartTransaction { modes, modifier, .. } => lower_begin(modes, modifier),
        SqlStatement::Commit { chain, end, modifier } => lower_commit(chain, end, modifier),
        SqlStatement::Rollback { chain, savepoint } => lower_rollback(chain, savepoint),
        other => Err(outside(statement_label(&other))),
    }
}
