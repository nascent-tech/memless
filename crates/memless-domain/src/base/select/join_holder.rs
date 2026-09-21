use crate::base::table::Table;
use crate::query::ColumnRef;

pub(crate) fn join_holder<'a>(from: &'a Table, joined: &'a Table, relation: &ColumnRef) -> Option<(&'a Table, bool)> {
    let name = relation.table.as_deref()?;
    if name == from.name {
        return Some((joined, true));
    }
    if name == joined.name {
        return Some((from, false));
    }
    None
}
