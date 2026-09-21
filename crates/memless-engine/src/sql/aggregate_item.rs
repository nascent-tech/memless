use super::aggregate_of::aggregate_of;
use super::item_kind::ItemKind;
use memless_domain::QueryRefusal;
use sqlparser::ast::Function;

pub(crate) fn aggregate_item(function: &Function, has_join: bool) -> Result<ItemKind, QueryRefusal> {
    Ok(ItemKind::Aggregate(aggregate_of(function, has_join)?))
}
