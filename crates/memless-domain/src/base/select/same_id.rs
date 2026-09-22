use crate::scalar::Scalar;
use crate::Id;

pub(crate) fn same_id(value: &Scalar, target: &Id) -> bool {
    Id::from_scalar(value.clone()).as_ref() == Some(target)
}
