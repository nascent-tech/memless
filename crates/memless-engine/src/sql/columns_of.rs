use super::column_of::column_of;
use super::item_kind::ItemKind;
use memless_domain::query::ColumnRef;

pub(crate) fn columns_of(kinds: Vec<ItemKind>) -> Vec<ColumnRef> {
    kinds.into_iter().filter_map(column_of).collect()
}
