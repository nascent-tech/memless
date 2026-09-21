use super::filter::Filter;

#[derive(Debug, Clone, PartialEq)]
pub struct Delete {
    pub table: String,
    pub filter: Option<Filter>,
}
