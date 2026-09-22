use super::collect_filter_node::collect_filter_node;
use crate::query::{ColumnRef, Filter};

pub(crate) fn filter_columns(filter: &Filter) -> Vec<&ColumnRef> {
    let mut stack = vec![filter];
    let mut columns = Vec::new();
    while let Some(node) = stack.pop() {
        collect_filter_node(node, &mut columns, &mut stack);
    }
    columns
}
