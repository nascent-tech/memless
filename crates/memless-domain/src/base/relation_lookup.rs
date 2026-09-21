use std::collections::HashSet;

use crate::scalar::Scalar;
use crate::Id;

pub(crate) fn contains_id(ids: &HashSet<&Id>, value: &Scalar) -> bool {
    match Id::from_scalar(value.clone()) {
        Some(wanted) => ids.contains(&wanted),
        None => false,
    }
}
