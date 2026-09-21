use std::ffi::c_char;

use super::kind::MemlessKind;
use super::payload::Payload;

/// # Safety
/// `payload.text` must be null or a writable pointer valid for the call.
pub(crate) unsafe fn write_text(payload: &Payload, pointer: *const c_char) -> MemlessKind {
    if !payload.text.is_null() {
        *payload.text = pointer;
    }
    MemlessKind::Text
}
