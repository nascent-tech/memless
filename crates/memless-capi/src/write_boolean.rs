use super::kind::MemlessKind;
use super::payload::Payload;

/// # Safety
/// `payload.boolean` must be null or a writable pointer valid for the call.
pub(crate) unsafe fn write_boolean(payload: &Payload, flag: bool) -> MemlessKind {
    if !payload.boolean.is_null() {
        *payload.boolean = i32::from(flag);
    }
    MemlessKind::Boolean
}
