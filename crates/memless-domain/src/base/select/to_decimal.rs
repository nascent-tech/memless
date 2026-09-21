use crate::scalar::Scalar;

pub(crate) fn to_decimal(value: &Scalar) -> Option<f64> {
    match value {
        Scalar::Decimal(number) => Some(*number),
        _ => None,
    }
}
