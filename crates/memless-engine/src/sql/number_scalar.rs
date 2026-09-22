use super::decimal_scalar::decimal_scalar;
use super::integer_scalar::integer_scalar;
use memless_domain::QueryRefusal;
use memless_domain::Scalar;

pub(crate) fn number_scalar(number: &str, negative: bool) -> Result<Scalar, QueryRefusal> {
    if number.contains('.') || number.contains(['e', 'E']) {
        return decimal_scalar(number, negative);
    }
    integer_scalar(number, negative)
}
