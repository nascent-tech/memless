use super::number_scalar::number_scalar;
use super::outside::outside;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, Value};

pub(crate) fn negated(expr: &Expr) -> Result<Scalar, QueryRefusal> {
    match expr {
        Expr::Value(Value::Number(number, _)) => number_scalar(number, true),
        _ => Err(outside("this expression")),
    }
}
