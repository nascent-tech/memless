use std::collections::HashSet;
use std::ffi::{c_char, CStr, CString};
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

use memless_capi::{
    memless_abi_version, memless_free_string, memless_load, memless_release, MemlessStatus,
};

fn fixture(name: &str) -> CString {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../harness/parity/fixtures");
    CString::new(format!("{base}/{name}")).expect("fixture path")
}

#[test]
fn reports_abi_version_one() {
    assert_eq!(memless_abi_version(), 1);
}

#[test]
fn loads_a_valid_file_and_writes_null_through_out_message() {
    let path = fixture("start.yaml");
    let mut handle: u64 = 0;
    let mut sentinel: c_char = 0;
    let mut message: *mut c_char = &mut sentinel;
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::Ok);
    assert!(handle >= 1);
    assert!(message.is_null());
    memless_release(handle);
}

#[test]
fn refuses_an_incoherent_file_leaving_the_handle_and_naming_the_rule() {
    let path = fixture("missing-id.yaml");
    let mut handle: u64 = 42;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::Refused);
    assert_eq!(handle, 42);
    assert!(!message.is_null());
    let text = unsafe { CStr::from_ptr(message) }.to_str().expect("utf-8").to_string();
    assert_eq!(text, "row 1 in \"users\" has no id");
    unsafe { memless_free_string(message) };
}

#[test]
fn rejects_a_null_path_writing_a_boundary_message() {
    let mut handle: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(ptr::null(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    assert!(!message.is_null());
    unsafe { memless_free_string(message) };
}

#[test]
fn rejects_a_null_out_handle_writing_a_boundary_message() {
    let path = fixture("start.yaml");
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(path.as_ptr(), ptr::null_mut(), &mut message) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    assert!(!message.is_null());
    unsafe { memless_free_string(message) };
}

#[test]
fn rejects_a_non_utf8_path_writing_a_boundary_message() {
    let path = CString::new(vec![0xFFu8, 0xFE]).expect("c string");
    let mut handle: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    assert!(!message.is_null());
    unsafe { memless_free_string(message) };
}

#[test]
fn returns_the_status_alone_when_out_message_is_null_on_ok() {
    let path = fixture("start.yaml");
    let mut handle: u64 = 0;
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::Ok);
    memless_release(handle);
}

#[test]
fn returns_the_status_alone_when_out_message_is_null_on_refused() {
    let path = fixture("missing-id.yaml");
    let mut handle: u64 = 0;
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::Refused);
}

#[test]
fn returns_the_status_alone_when_out_message_is_null_on_invalid_argument() {
    let status = unsafe { memless_load(ptr::null(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
}

#[test]
fn allocates_handles_without_reusing_a_released_one() {
    let path = fixture("start.yaml");
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    unsafe { memless_load(path.as_ptr(), &mut first, &mut message) };
    unsafe { memless_load(path.as_ptr(), &mut second, &mut message) };
    assert!(first >= 1 && second > first);
    memless_release(first);
    let mut third: u64 = 0;
    unsafe { memless_load(path.as_ptr(), &mut third, &mut message) };
    assert!(third > second);
    memless_release(second);
    memless_release(third);
}

#[test]
fn allocates_distinct_handles_across_threads() {
    let path = fixture("start.yaml").to_str().expect("utf-8").to_string();
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let mut joins = Vec::new();
    for _ in 0..8 {
        let path = path.clone();
        let seen = Arc::clone(&seen);
        joins.push(thread::spawn(move || {
            let path = CString::new(path).expect("c string");
            for _ in 0..100 {
                let mut handle: u64 = 0;
                let mut message: *mut c_char = ptr::null_mut();
                let status = unsafe { memless_load(path.as_ptr(), &mut handle, &mut message) };
                assert_eq!(status, MemlessStatus::Ok);
                seen.lock().expect("lock").insert(handle);
                memless_release(handle);
            }
        }));
    }
    for join in joins {
        join.join().expect("thread");
    }
    assert_eq!(seen.lock().expect("lock").len(), 800);
}

#[test]
fn ignores_unknown_and_zero_handles_on_release() {
    memless_release(0);
    memless_release(999_999);
}

#[test]
fn ignores_null_on_free_string() {
    unsafe { memless_free_string(ptr::null_mut()) };
}
