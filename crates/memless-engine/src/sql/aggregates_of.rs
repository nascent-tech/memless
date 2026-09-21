use super::aggregate_only::aggregate_only;
use super::item_kind::ItemKind;
use memless_domain::query::Aggregate;

pub(crate) fn aggregates_of(kinds: Vec<ItemKind>) -> Vec<Aggregate> {
    kinds.into_iter().filter_map(aggregate_only).collect()
}
