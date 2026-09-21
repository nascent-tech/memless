use std::ffi::c_char;
use std::ptr;

use crate::column_ptr::column_ptr;
use crate::compute::compute;
use crate::execute_compute::execute_compute;
use crate::ffi::free_string;
use crate::instances::discard;
use crate::outcome::Outcome;
use crate::payload::Payload;
use crate::query_compute::query_compute;
use crate::read_cell::read_cell;
use crate::results::{discard_result, with_result};
use crate::write::write_outcome;

mod apply;
mod c_text;
mod cell;
mod cell_at;
mod column_ptr;
mod compute;
mod execute_compute;
mod ffi;
mod instances;
mod run_execute;
mod kind;
mod message_ptr;
mod outcome;
mod payload;
mod prepare;
mod prepared;
mod query_compute;
mod read_cell;
mod resolve_cell;
mod resolved;
mod results;
mod run_query;
mod status;
mod write;
mod write_boolean;
mod write_decimal;
mod write_integer;
mod write_text;

pub use ffi::{guard, own_message};
pub use kind::MemlessKind;
pub use status::{MemlessHandle, MemlessResult, MemlessStatus};

const ABI_VERSION: u32 = 3;

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

/// Runs `sql` against `handle`. See `include/memless.h` for the contract.
///
/// # Safety
/// `sql`, `out_result` and `out_message` follow the header's contract.
#[no_mangle]
pub unsafe extern "C" fn memless_query(
    handle: MemlessHandle,
    sql: *const c_char,
    out_result: *mut MemlessResult,
    out_message: *mut *mut c_char,
) -> MemlessStatus {
    let outcome = guard(Outcome::internal(), || unsafe { query_compute(handle, sql, out_result.is_null()) });
    write_outcome(out_result, out_message, outcome)
}

/// Runs a write `sql` against `handle`. See `include/memless.h` for the contract.
///
/// # Safety
/// `sql`, `out_affected` and `out_message` follow the header's contract.
#[no_mangle]
pub unsafe extern "C" fn memless_execute(
    handle: MemlessHandle,
    sql: *const c_char,
    out_affected: *mut u64,
    out_message: *mut *mut c_char,
) -> MemlessStatus {
    let outcome = guard(Outcome::internal(), || unsafe { execute_compute(handle, sql) });
    write_outcome(out_affected, out_message, outcome)
}

#[no_mangle]
pub extern "C" fn memless_result_column_count(result: MemlessResult) -> u64 {
    guard(0, || with_result(result, |prepared| prepared.columns.len() as u64).unwrap_or(0))
}

#[no_mangle]
pub extern "C" fn memless_result_row_count(result: MemlessResult) -> u64 {
    guard(0, || with_result(result, |prepared| prepared.rows.len() as u64).unwrap_or(0))
}

/// Borrowed until `memless_result_release`; the caller never frees it.
#[no_mangle]
pub extern "C" fn memless_result_column(result: MemlessResult, index: u64) -> *const c_char {
    guard(ptr::null(), || with_result(result, |prepared| column_ptr(prepared, index)).unwrap_or(ptr::null()))
}

/// Writes the cell kind and, for a present scalar, one `out_*` payload.
///
/// # Safety
/// Each `out_*` pointer must be null or writable for the call; a borrowed text
/// pointer stays valid until `memless_result_release`.
#[no_mangle]
pub unsafe extern "C" fn memless_result_cell(
    result: MemlessResult,
    row: u64,
    column: u64,
    out_integer: *mut i64,
    out_decimal: *mut f64,
    out_boolean: *mut i32,
    out_text: *mut *const c_char,
) -> MemlessKind {
    let payload = Payload { integer: out_integer, decimal: out_decimal, boolean: out_boolean, text: out_text };
    guard(MemlessKind::Absent, || unsafe { read_cell(result, row, column, payload) })
}

#[no_mangle]
pub extern "C" fn memless_result_release(result: MemlessResult) {
    guard((), || discard_result(result));
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
