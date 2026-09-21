use super::join_label::join_label;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Expr, JoinConstraint, JoinOperator};

pub(crate) fn join_on(operator: &JoinOperator) -> Result<&Expr, QueryRefusal> {
    match operator {
        JoinOperator::Inner(JoinConstraint::On(expr)) => Ok(expr),
        JoinOperator::Inner(JoinConstraint::Using(_)) => Err(outside("JOIN USING")),
        JoinOperator::Inner(JoinConstraint::Natural) => Err(outside("NATURAL JOIN")),
        JoinOperator::Inner(_) => Err(outside("JOIN without ON")),
        other => Err(outside(join_label(other))),
    }
}
