use super::item_kind::ItemKind;
use memless_domain::query::Aggregate;

pub(crate) fn aggregate_only(kind: ItemKind) -> Option<Aggregate> {
    match kind {
        ItemKind::Aggregate(aggregate) => Some(aggregate),
        _ => None,
    }
}
