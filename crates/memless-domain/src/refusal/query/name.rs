use super::QueryRefusal;
use super::QueryRefusal::{JoinNotRelation, UnknownColumn, UnknownTable};

pub(crate) fn name_message(refusal: &QueryRefusal) -> Option<String> {
    match refusal {
        UnknownTable { table } => Some(format!("no table {table:?}")),
        UnknownColumn { table, column } => Some(format!("no column {column:?} in table {table:?}")),
        JoinNotRelation { table, column, target } => Some(format!(
            "{column:?} of {table:?} is not a guessed relation to the id of {target:?}"
        )),
        _ => None,
    }
}
