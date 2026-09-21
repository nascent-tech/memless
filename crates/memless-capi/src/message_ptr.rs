use std::ffi::c_char;
use std::ptr;

use crate::ffi::own_message;

pub(crate) fn message_ptr(message: Option<String>) -> *mut c_char {
    match message {
        Some(text) => own_message(&text),
        None => ptr::null_mut(),
    }
}
