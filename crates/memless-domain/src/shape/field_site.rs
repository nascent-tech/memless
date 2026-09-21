use crate::refusal::RowLabel;
use crate::scalar::Scalar;

pub(crate) type ShapedRow = Vec<(String, Scalar)>;

pub(crate) type RowAcc = (Vec<String>, ShapedRow);

pub(crate) struct FieldSite<'a> {
    pub table: &'a str,
    pub position: usize,
    pub label: RowLabel,
}
