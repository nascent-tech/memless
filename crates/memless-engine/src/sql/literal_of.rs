use super::negated::negated;
use super::outside::outside;
use super::value_scalar::value_scalar;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, UnaryOperator};

pub(crate) fn literal_of(expr: &Expr) -> Result<Scalar, QueryRefusal> {
    match expr {
        Expr::Value(value) => value_scalar(value),
        Expr::UnaryOp { op: UnaryOperator::Minus, expr } => negated(expr),
        _ => Err(outside("this literal")),
    }
}
