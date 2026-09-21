#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnRef {
    pub table: Option<String>,
    pub column: String,
}
