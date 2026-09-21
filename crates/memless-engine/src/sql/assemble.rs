use super::aggregates_of::aggregates_of;
use super::columns_of::columns_of;
use super::is_aggregate::is_aggregate;
use super::is_column::is_column;
use super::outside::outside;
use memless_domain::query::Items;
use memless_domain::QueryRefusal;
use super::item_kind::ItemKind;

pub(crate) fn assemble(kinds: Vec<ItemKind>) -> Result<Items, QueryRefusal> {
    if kinds.iter().all(is_column) {
        return Ok(Items::Columns(columns_of(kinds)));
    }
    if kinds.iter().all(is_aggregate) {
        return Ok(Items::Aggregates(aggregates_of(kinds)));
    }
    Err(outside("a column next to an aggregate"))
}
