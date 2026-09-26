use super::QueryRefusal;
use super::QueryRefusal::{OrderMixedTypes, SumNotNumber, SumOverflow};

pub(crate) fn execution_message(refusal: &QueryRefusal) -> Option<String> {
    match refusal {
        SumNotNumber { table, column, row } => Some(format!(
            "cannot sum {column:?} of {table:?} at row {row}"
        )),
        SumOverflow { table, column } => Some(format!("sum of {column:?} in {table:?} overflows")),
        OrderMixedTypes { table, column, first, second } => Some(format!(
            "cannot order by {column:?} of {table:?}: row {first} and row {second} differ in type"
        )),
        _ => None,
    }
}
