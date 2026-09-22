use std::cmp::Ordering;

use crate::query::Op;
use crate::scalar::scalar_order::compare_same_type;
use crate::scalar::Scalar;

pub(crate) fn holds(op: Op, value: &Scalar, literal: &Scalar) -> bool {
    let order = compare_same_type(value, literal);
    match op {
        Op::Eq => order == Some(Ordering::Equal),
        Op::Ne => matches!(order, Some(Ordering::Less) | Some(Ordering::Greater)),
        Op::Lt => order == Some(Ordering::Less),
        Op::Le => matches!(order, Some(Ordering::Less) | Some(Ordering::Equal)),
        Op::Gt => order == Some(Ordering::Greater),
        Op::Ge => matches!(order, Some(Ordering::Greater) | Some(Ordering::Equal)),
    }
}
