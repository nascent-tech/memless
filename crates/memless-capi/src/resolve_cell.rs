use super::cell::Cell;
use super::cell_at::cell_at;
use super::prepared::Prepared;
use super::resolved::Resolved;

pub(crate) fn resolve_cell(prepared: &Prepared, row: u64, column: u64) -> Resolved {
    match cell_at(prepared, row, column) {
        Some(Cell::Text(text)) => Resolved::Text(text.as_ptr()),
        Some(Cell::Integer(number)) => Resolved::Integer(*number),
        Some(Cell::Decimal(number)) => Resolved::Decimal(*number),
        Some(Cell::Boolean(flag)) => Resolved::Boolean(*flag),
        _ => Resolved::Absent,
    }
}
