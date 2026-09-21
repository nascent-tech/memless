use std::cmp::Ordering;

use super::Scalar;

pub(crate) fn compare_same_type(left: &Scalar, right: &Scalar) -> Option<Ordering> {
    match (left, right) {
        (Scalar::Text(a), Scalar::Text(b)) => Some(a.as_bytes().cmp(b.as_bytes())),
        (Scalar::Integer(a), Scalar::Integer(b)) => Some(a.cmp(b)),
        (Scalar::Decimal(a), Scalar::Decimal(b)) => a.partial_cmp(b),
        (Scalar::Boolean(a), Scalar::Boolean(b)) => Some(a.cmp(b)),
        _ => None,
    }
}
