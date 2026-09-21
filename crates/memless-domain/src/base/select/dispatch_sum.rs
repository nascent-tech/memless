use super::not_number::not_number;
use super::plan::Plan;
use super::sum_decimals::sum_decimals;
use super::sum_integers::sum_integers;
use crate::query::ColumnRef;
use crate::refusal::{QueryRefusal, RowLabel};
use crate::scalar::Scalar;

pub(crate) fn dispatch_sum(
    plan: &Plan,
    column: &ColumnRef,
    first: &Scalar,
    present: &[(RowLabel, Scalar)],
) -> Result<Option<Scalar>, QueryRefusal> {
    match first {
        Scalar::Integer(_) => sum_integers(plan, column, present),
        Scalar::Decimal(_) => sum_decimals(plan, column, present),
        _ => Err(not_number(plan, column, &present[0].0)),
    }
}
