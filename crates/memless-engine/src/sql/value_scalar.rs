use super::number_scalar::number_scalar;
use super::outside::outside;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::Value;

pub(crate) fn value_scalar(value: &Value) -> Result<Scalar, QueryRefusal> {
    match value {
        Value::SingleQuotedString(text) => Ok(Scalar::Text(text.clone())),
        Value::Number(number, _) => number_scalar(number, false),
        Value::Boolean(flag) => Ok(Scalar::Boolean(*flag)),
        Value::Null => Err(outside("NULL literal")),
        _ => Err(outside("this literal")),
    }
}
