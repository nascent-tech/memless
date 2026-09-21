use super::assign::apply_assignments;
use crate::base::build_row::build_row;
use crate::base::row::Row;
use crate::refusal::Refusal;
use crate::scalar::Scalar;

pub(crate) fn rebuild_row(
    table: &str,
    position: usize,
    row: &Row,
    assignments: &[(String, Option<Scalar>)],
) -> Result<Row, Refusal> {
    let columns = apply_assignments(&row.columns, assignments);
    let built = build_row(table, position + 1, &columns)?;
    Ok(built)
}
