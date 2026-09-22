use crate::query::Insert;
use crate::refusal::Refusal;
use crate::refusal::WriteRefusal::ColumnCountMismatch;

pub(crate) fn check_count(spec: &Insert) -> Result<(), Refusal> {
    if spec.columns.len() == spec.values.len() {
        return Ok(());
    }
    Err(Refusal::from(ColumnCountMismatch {
        table: spec.table.clone(),
        columns: spec.columns.len(),
        values: spec.values.len(),
    }))
}
