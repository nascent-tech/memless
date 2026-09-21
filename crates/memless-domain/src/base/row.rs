use crate::scalar::Scalar;
use crate::Id;

pub(crate) struct Row {
    pub(crate) id: Id,
    pub(crate) columns: Vec<(String, Scalar)>,
}
