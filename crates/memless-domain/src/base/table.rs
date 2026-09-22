use super::row::Row;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) rows: Vec<Row>,
}
