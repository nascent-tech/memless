use super::row::Row;

pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) rows: Vec<Row>,
}
