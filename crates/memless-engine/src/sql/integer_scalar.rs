use super::invalid::invalid;
use super::signed_text::signed_text;
use memless_domain::QueryRefusal;
use memless_domain::Scalar;

pub(crate) fn integer_scalar(number: &str, negative: bool) -> Result<Scalar, QueryRefusal> {
    let value: i64 = signed_text(number, negative)
        .parse()
        .map_err(|_| invalid("integer literal out of range"))?;
    Ok(Scalar::Integer(value))
}
