use super::not_number::not_number;
use super::overflow::overflow;
use super::plan::Plan;
use super::to_integer::to_integer;
use crate::query::ColumnRef;
use crate::refusal::{QueryRefusal, RowLabel};
use crate::scalar::Scalar;

pub(crate) fn sum_integers(
    plan: &Plan,
    column: &ColumnRef,
    present: &[(RowLabel, Scalar)],
) -> Result<Option<Scalar>, QueryRefusal> {
    let mut total: i64 = 0;
    for (row, value) in present {
        let number = to_integer(value).ok_or_else(|| not_number(plan, column, row))?;
        total = total.checked_add(number).ok_or_else(|| overflow(plan, column))?;
    }
    Ok(Some(Scalar::Integer(total)))
}
