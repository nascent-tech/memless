use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, UnwindSafe};
use std::ptr;

pub(crate) enum FfiError {
    NullPointer,
    InvalidUtf8,
}

impl FfiError {
    pub(crate) fn describe(&self) -> &'static str {
        match self {
            FfiError::NullPointer => "null pointer argument",
            FfiError::InvalidUtf8 => "path is not valid UTF-8",
        }
    }
}

/// # Safety
/// `path` must be null or a valid NUL-terminated C string.
pub(crate) unsafe fn own_path(path: *const c_char) -> Result<String, FfiError> {
    if path.is_null() {
        return Err(FfiError::NullPointer);
    }
    CStr::from_ptr(path).to_str().map(str::to_string).map_err(|_| FfiError::InvalidUtf8)
}

pub fn own_message(text: &str) -> *mut c_char {
    let sanitised = text.replace('\0', "\u{FFFD}");
    match CString::new(sanitised) {
        Ok(owned) => owned.into_raw(),
        // Unreachable: the interior NUL was just replaced; kept total for safety.
        Err(_) => ptr::null_mut(),
    }
}

/// # Safety
/// `message` must be null or a pointer returned by `own_message` and not yet freed.
pub(crate) unsafe fn free_string(message: *mut c_char) {
    if !message.is_null() {
        drop(CString::from_raw(message));
    }
}

pub fn guard<R>(fallback: R, body: impl FnOnce() -> R + UnwindSafe) -> R {
    catch_unwind(body).unwrap_or(fallback)
}
