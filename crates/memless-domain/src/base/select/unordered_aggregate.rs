use crate::query::{Items, Select};
use crate::refusal::QueryRefusal;

pub(crate) fn unordered_aggregate(query: &Select) -> Result<(), QueryRefusal> {
    if query.order.is_empty() || !matches!(query.items, Items::Aggregates(_)) {
        return Ok(());
    }
    Err(QueryRefusal::OutsideSubset { construct: "ORDER BY with an aggregate".to_string() })
}
