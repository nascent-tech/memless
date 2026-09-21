use super::item_kind::ItemKind;
use memless_domain::query::ColumnRef;

pub(crate) fn column_of(kind: ItemKind) -> Option<ColumnRef> {
    match kind {
        ItemKind::Column(column) => Some(column),
        _ => None,
    }
}
