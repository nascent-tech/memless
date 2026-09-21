use crate::scalar::Scalar;
use crate::Id;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Row {
    pub(crate) id: Id,
    pub(crate) columns: Vec<(String, Scalar)>,
}
