use std::ffi::c_char;

use crate::compute::compute;
use crate::ffi::free_string;
use crate::instances::discard;
use crate::outcome::Outcome;
use crate::write::write_outcome;

mod compute;
mod ffi;
mod instances;
mod message_ptr;
mod outcome;
mod status;
mod write;

pub use ffi::{guard, own_message};
pub use status::{MemlessHandle, MemlessStatus};

const ABI_VERSION: u32 = 1;

#[no_mangle]
pub extern "C" fn memless_abi_version() -> u32 {
    guard(ABI_VERSION, || ABI_VERSION)
}

/// Loads an instance on `path`. See `include/memless.h` for the contract.
///
/// # Safety
/// `path`, `out_handle` and `out_message` follow the header's contract.
#[no_mangle]
pub unsafe extern "C" fn memless_load(
    path: *const c_char,
    out_handle: *mut MemlessHandle,
    out_message: *mut *mut c_char,
) -> MemlessStatus {
    let outcome = guard(Outcome::internal(), || unsafe { compute(path, out_handle.is_null()) });
    write_outcome(out_handle, out_message, outcome)
}

#[no_mangle]
pub extern "C" fn memless_release(handle: MemlessHandle) {
    guard((), || discard(handle));
}

/// # Safety
/// `message` must be null or a pointer returned through `out_message`.
#[no_mangle]
pub unsafe extern "C" fn memless_free_string(message: *mut c_char) {
    guard((), || unsafe { free_string(message) });
}
