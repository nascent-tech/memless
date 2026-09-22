use super::filter::Filter;
use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    pub table: String,
    pub assignments: Vec<(String, Option<Scalar>)>,
    pub filter: Option<Filter>,
}
