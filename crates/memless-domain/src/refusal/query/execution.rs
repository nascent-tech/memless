use super::QueryRefusal;
use super::QueryRefusal::{SumNotNumber, SumOverflow};

pub(crate) fn execution_message(refusal: &QueryRefusal) -> Option<String> {
    match refusal {
        SumNotNumber { table, column, row } => Some(format!(
            "cannot sum {column:?} of {table:?} at row {row}"
        )),
        SumOverflow { table, column } => Some(format!("sum of {column:?} in {table:?} overflows")),
        _ => None,
    }
}
