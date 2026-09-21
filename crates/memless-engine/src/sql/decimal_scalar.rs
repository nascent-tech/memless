use super::invalid::invalid;
use super::signed_text::signed_text;
use memless_domain::QueryRefusal;
use memless_domain::Scalar;

pub(crate) fn decimal_scalar(number: &str, negative: bool) -> Result<Scalar, QueryRefusal> {
    let value: f64 = signed_text(number, negative)
        .parse()
        .map_err(|_| invalid("decimal literal is invalid"))?;
    Ok(Scalar::Decimal(value))
}
