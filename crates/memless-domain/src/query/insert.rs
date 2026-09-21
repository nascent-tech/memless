use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Insert {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Option<Scalar>>,
}
