use crate::scalar::Scalar;

pub(crate) fn to_integer(value: &Scalar) -> Option<i64> {
    match value {
        Scalar::Integer(number) => Some(*number),
        _ => None,
    }
}
