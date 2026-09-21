use super::kind::MemlessKind;
use super::payload::Payload;

/// # Safety
/// `payload.integer` must be null or a writable pointer valid for the call.
pub(crate) unsafe fn write_integer(payload: &Payload, number: i64) -> MemlessKind {
    if !payload.integer.is_null() {
        *payload.integer = number;
    }
    MemlessKind::Integer
}
