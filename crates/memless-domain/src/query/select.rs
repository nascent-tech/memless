use super::filter::Filter;
use super::items::Items;
use super::join::Join;

#[derive(Debug, Clone, PartialEq)]
pub struct Select {
    pub items: Items,
    pub from: String,
    pub join: Option<Join>,
    pub filter: Option<Filter>,
}
