use super::collect_node::collect_node;
use crate::query::{ColumnRef, Filter};

pub(crate) fn columns(filter: &Filter) -> Vec<&ColumnRef> {
    let mut stack = vec![filter];
    let mut columns = Vec::new();
    while let Some(node) = stack.pop() {
        collect_node(node, &mut columns, &mut stack);
    }
    columns
}
