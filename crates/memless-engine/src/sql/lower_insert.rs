use super::insert_columns::insert_columns;
use super::insert_row::insert_row;
use super::reject_insert_options::reject_insert_options;
use super::reject_insert_shape::reject_insert_shape;
use super::table_object::table_object_name;
use memless_domain::query::{Insert, Write};
use memless_domain::QueryRefusal;
use sqlparser::ast::Insert as SqlInsert;

pub(crate) fn lower_insert(insert: SqlInsert) -> Result<Write, QueryRefusal> {
    reject_insert_options(&insert)?;
    reject_insert_shape(&insert)?;
    let table = table_object_name(&insert.table)?;
    let columns = insert_columns(&insert)?;
    let values = insert_row(insert.source)?;
    Ok(Write::Insert(Insert { table, columns, values }))
}
