use std::ffi::c_char;

use crate::ffi::own_path;
use crate::outcome::Outcome;
use crate::run_query::run_query;

/// # Safety
/// `sql` must be null or a valid NUL-terminated C string.
pub(crate) unsafe fn query_compute(handle: u64, sql: *const c_char, out_result_null: bool) -> Outcome {
    if out_result_null {
        return Outcome::invalid("null out_result pointer");
    }
    match own_path(sql) {
        Ok(owned) => run_query(handle, &owned),
        Err(error) => Outcome::invalid(error.describe()),
    }
}
