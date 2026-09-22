use std::ffi::c_char;
use std::ptr;

use super::prepared::Prepared;

pub(crate) fn column_ptr(prepared: &Prepared, index: u64) -> *const c_char {
    match usize::try_from(index).ok().and_then(|index| prepared.columns.get(index)) {
        Some(name) => name.as_ptr(),
        None => ptr::null(),
    }
}
