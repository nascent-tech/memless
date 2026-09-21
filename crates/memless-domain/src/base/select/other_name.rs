use crate::base::table::Table;

pub(crate) fn other_name(from: &Table, joined: &Table, table: &str) -> String {
    if table == from.name {
        return joined.name.clone();
    }
    from.name.clone()
}
