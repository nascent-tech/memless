use super::not_number::not_number;
use super::plan::Plan;
use super::to_decimal::to_decimal;
use crate::query::ColumnRef;
use crate::refusal::{QueryRefusal, RowLabel};
use crate::scalar::Scalar;

pub(crate) fn sum_decimals(
    plan: &Plan,
    column: &ColumnRef,
    present: &[(RowLabel, Scalar)],
) -> Result<Option<Scalar>, QueryRefusal> {
    let mut total = 0.0;
    for (row, value) in present {
        total += to_decimal(value).ok_or_else(|| not_number(plan, column, row))?;
    }
    Ok(Some(Scalar::Decimal(total)))
}
