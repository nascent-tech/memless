use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Rows {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<Scalar>>>,
}
