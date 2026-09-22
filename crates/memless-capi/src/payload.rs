use std::ffi::c_char;

pub(crate) struct Payload {
    pub integer: *mut i64,
    pub decimal: *mut f64,
    pub boolean: *mut i32,
    pub text: *mut *const c_char,
}
