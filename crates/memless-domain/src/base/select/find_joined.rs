use super::resolve::find_table;
use crate::base::table::Table;
use crate::query::Select;
use crate::refusal::QueryRefusal;

pub(crate) fn find_joined<'a>(tables: &'a [Table], query: &Select) -> Result<Option<&'a Table>, QueryRefusal> {
    match &query.join {
        Some(join) => find_table(tables, &join.table).map(Some),
        None => Ok(None),
    }
}
