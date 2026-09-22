use sqlparser::ast::{ObjectName, TableFactor};

pub(crate) fn plain_table(relation: &TableFactor) -> Option<&ObjectName> {
    let TableFactor::Table {
        name, alias: None, args: None, version: None, sample: None, with_hints, partitions, ..
    } = relation
    else {
        return None;
    };
    if with_hints.is_empty() && partitions.is_empty() {
        return Some(name);
    }
    None
}
