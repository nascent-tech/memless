use super::candidate::Candidate;
use super::col::Col;
use super::col_value::col_value;
use crate::scalar::Scalar;

pub(crate) fn row_cells<'a>(cols: &[Col], candidate: &Candidate<'a>) -> Vec<Option<Scalar>> {
    cols.iter().map(|col| col_value(candidate, col)).collect()
}
