use super::filter::Filter;
use super::items::Items;
use super::join::Join;
use super::order_key::OrderKey;

#[derive(Debug, Clone, PartialEq)]
pub struct Select {
    pub items: Items,
    pub from: String,
    pub join: Option<Join>,
    pub filter: Option<Filter>,
    pub order: Vec<OrderKey>,
}
