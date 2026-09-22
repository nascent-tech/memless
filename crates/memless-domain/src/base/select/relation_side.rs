use crate::query::ColumnRef;

pub(crate) fn relation_side<'a>(left: &'a ColumnRef, right: &'a ColumnRef) -> &'a ColumnRef {
    if left.column == "id" && right.column != "id" {
        return right;
    }
    left
}
