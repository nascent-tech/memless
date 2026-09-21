use super::delete_table::delete_table;
use super::lower_selection::lower_selection;
use super::reject_delete_tail::reject_delete_tail;
use memless_domain::query::{Delete, Write};
use memless_domain::QueryRefusal;
use sqlparser::ast::Delete as SqlDelete;

pub(crate) fn lower_delete(delete: SqlDelete) -> Result<Write, QueryRefusal> {
    reject_delete_tail(&delete)?;
    let table = delete_table(&delete)?;
    let filter = lower_selection(&delete.selection, false)?;
    Ok(Write::Delete(Delete { table, filter }))
}
