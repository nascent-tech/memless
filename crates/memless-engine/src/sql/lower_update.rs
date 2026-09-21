use super::assignments_of::assignments_of;
use super::invalid::invalid;
use super::lower_selection::lower_selection;
use super::reject_update_tail::reject_update_tail;
use super::sole_table::sole_table;
use memless_domain::query::{Update, Write};
use memless_domain::QueryRefusal;
use sqlparser::ast::Statement as SqlStatement;

pub(crate) fn lower_update(statement: SqlStatement) -> Result<Write, QueryRefusal> {
    let SqlStatement::Update { table, assignments, from, selection, returning, or } = statement else {
        return Err(invalid("not an UPDATE"));
    };
    reject_update_tail(from.is_some(), returning.is_some(), or.is_some())?;
    let name = sole_table(&table)?;
    let sets = assignments_of(&assignments)?;
    let filter = lower_selection(&selection, false)?;
    Ok(Write::Update(Update { table: name, assignments: sets, filter }))
}
