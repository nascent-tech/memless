use crate::query::{ColumnRef, Select};

pub(crate) fn join_columns(query: &Select) -> Vec<&ColumnRef> {
    match &query.join {
        Some(join) => vec![&join.left, &join.right],
        None => Vec::new(),
    }
}
