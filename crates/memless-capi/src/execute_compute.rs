use std::ffi::c_char;

use crate::ffi::own_path;
use crate::outcome::Outcome;
use crate::run_execute::run_execute;

/// # Safety
/// `sql` must be null or a valid NUL-terminated C string.
pub(crate) unsafe fn execute_compute(handle: u64, sql: *const c_char) -> Outcome {
    match own_path(sql) {
        Ok(owned) => run_execute(handle, &owned),
        Err(error) => Outcome::invalid(error.describe()),
    }
}
