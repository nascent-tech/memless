use super::duplicate::first_duplicate;
use crate::refusal::Refusal;
use crate::refusal::WriteRefusal::ColumnRepeated;

pub(crate) fn check_unique(table: &str, columns: &[String]) -> Result<(), Refusal> {
    match first_duplicate(columns) {
        Some(column) => Err(Refusal::from(ColumnRepeated { table: table.to_string(), column })),
        None => Ok(()),
    }
}
