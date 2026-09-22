use super::kind::MemlessKind;
use super::payload::Payload;

/// # Safety
/// `payload.decimal` must be null or a writable pointer valid for the call.
pub(crate) unsafe fn write_decimal(payload: &Payload, number: f64) -> MemlessKind {
    if !payload.decimal.is_null() {
        *payload.decimal = number;
    }
    MemlessKind::Decimal
}
