use super::apply::apply;
use super::kind::MemlessKind;
use super::payload::Payload;
use super::resolve_cell::resolve_cell;
use super::resolved::Resolved;
use super::results::with_result;

/// # Safety
/// The pointers in `payload` must each be null or writable for the call.
pub(crate) unsafe fn read_cell(result: u64, row: u64, column: u64, payload: Payload) -> MemlessKind {
    let resolved = with_result(result, |prepared| resolve_cell(prepared, row, column));
    apply(resolved.unwrap_or(Resolved::Absent), &payload)
}
