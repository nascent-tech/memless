use super::literal_of::literal_of;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, Value};

pub(crate) fn value_of(expr: &Expr) -> Result<Option<Scalar>, QueryRefusal> {
    if matches!(expr, Expr::Value(Value::Null)) {
        return Ok(None);
    }
    literal_of(expr).map(Some)
}
