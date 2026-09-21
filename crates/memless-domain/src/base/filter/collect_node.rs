use crate::query::{ColumnRef, Filter};

pub(crate) fn collect_node<'a>(
    node: &'a Filter,
    columns: &mut Vec<&'a ColumnRef>,
    stack: &mut Vec<&'a Filter>,
) {
    match node {
        Filter::Compare(compare) => columns.push(&compare.column),
        Filter::IsNull(column) | Filter::IsNotNull(column) => columns.push(column),
        Filter::And(left, right) | Filter::Or(left, right) => stack.extend([left.as_ref(), right.as_ref()]),
    }
}
