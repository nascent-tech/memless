use std::collections::{HashMap, HashSet};

use super::table::Table;
use crate::Id;

pub(crate) type IdIndex<'a> = HashMap<&'a str, HashSet<&'a Id>>;

pub(crate) fn build_index(tables: &[Table]) -> IdIndex<'_> {
    tables
        .iter()
        .map(|table| (table.name.as_str(), table.rows.iter().map(|row| &row.id).collect()))
        .collect()
}
