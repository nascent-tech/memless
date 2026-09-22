use super::cell::Cell;
use super::prepared::Prepared;

pub(crate) fn cell_at(prepared: &Prepared, row: u64, column: u64) -> Option<&Cell> {
    let row = usize::try_from(row).ok()?;
    let column = usize::try_from(column).ok()?;
    prepared.rows.get(row)?.get(column)
}
