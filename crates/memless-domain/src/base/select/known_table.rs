use crate::base::table::Table;

pub(crate) fn known_table(from: &Table, joined: Option<&Table>, name: &str) -> bool {
    name == from.name || joined.map(|table| table.name == name).unwrap_or(false)
}
