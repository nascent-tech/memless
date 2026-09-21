use super::item_kind::ItemKind;

pub(crate) fn is_column(kind: &ItemKind) -> bool {
    matches!(kind, ItemKind::Column(_))
}
