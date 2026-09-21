use super::item_kind::ItemKind;

pub(crate) fn is_aggregate(kind: &ItemKind) -> bool {
    matches!(kind, ItemKind::Aggregate(_))
}
