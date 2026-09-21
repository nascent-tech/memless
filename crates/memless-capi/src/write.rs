use std::ffi::c_char;

use crate::message_ptr::message_ptr;
use crate::outcome::Outcome;
use crate::status::MemlessStatus;

/// # Safety
/// `out_handle` and `out_message` must each be null or a valid, writable pointer.
pub(crate) unsafe fn write_outcome(
    out_handle: *mut u64,
    out_message: *mut *mut c_char,
    outcome: Outcome,
) -> MemlessStatus {
    if let (false, Some(handle)) = (out_handle.is_null(), outcome.handle) {
        *out_handle = handle;
    }
    if !out_message.is_null() {
        *out_message = message_ptr(outcome.message);
    }
    outcome.status
}
