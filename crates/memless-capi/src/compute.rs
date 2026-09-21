use std::ffi::c_char;

use memless_engine::{load, read};

use crate::ffi::own_path;
use crate::instances::store;
use crate::outcome::Outcome;

/// # Safety
/// `path` must be null or a valid NUL-terminated C string.
pub(crate) unsafe fn compute(path: *const c_char, out_handle_null: bool) -> Outcome {
    if out_handle_null {
        return Outcome::invalid("null out_handle pointer");
    }
    let owned = match own_path(path) {
        Ok(owned) => owned,
        Err(error) => return Outcome::invalid(error.describe()),
    };
    match load(read, &owned) {
        Ok(base) => Outcome::accepted(store(base)),
        Err(refusal) => Outcome::refused(refusal.to_string()),
    }
}
