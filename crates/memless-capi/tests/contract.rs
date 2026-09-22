use std::collections::HashSet;
use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use memless_capi::{
    memless_abi_version, memless_execute, memless_free_string, memless_load, memless_query,
    memless_release, memless_result_cell, memless_result_column, memless_result_column_count,
    memless_result_release, memless_result_row_count, MemlessKind, MemlessStatus,
};

static EXEC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixture(name: &str) -> CString {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../harness/parity/fixtures");
    CString::new(format!("{base}/{name}")).expect("fixture path")
}

fn fixture_source(name: &str) -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../harness/parity/fixtures")).join(name)
}

fn temp_workdir() -> PathBuf {
    let dir = std::env::temp_dir()
        .join("memless-capi-exec")
        .join(format!("w-{}", EXEC_COUNTER.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn load_copy(name: &str) -> (u64, PathBuf) {
    let dir = temp_workdir();
    let dest = dir.join(name);
    std::fs::copy(fixture_source(name), &dest).expect("copy fixture");
    let cpath = CString::new(dest.to_str().expect("utf-8")).expect("c string");
    let mut handle: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(cpath.as_ptr(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::Ok);
    (handle, dir)
}

fn run_execute(handle: u64, sql: &str) -> (MemlessStatus, u64) {
    let csql = CString::new(sql).expect("c string");
    let mut affected: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_execute(handle, csql.as_ptr(), &mut affected, &mut message) };
    if !message.is_null() {
        unsafe { memless_free_string(message) };
    }
    (status, affected)
}

#[test]
fn reports_abi_version_four() {
    assert_eq!(memless_abi_version(), 4);
}

fn name_of(handle: u64, id: &str) -> String {
    let (status, result) = run_query(handle, &format!("SELECT name FROM users WHERE id = '{id}'"));
    assert_eq!(status, MemlessStatus::Ok);
    let mut text: *const c_char = ptr::null();
    unsafe { memless_result_cell(result, 0, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), &mut text) };
    let value = unsafe { CStr::from_ptr(text) }.to_str().expect("utf-8").to_string();
    memless_result_release(result);
    value
}

fn execute_message(handle: u64, sql: &str) -> (MemlessStatus, String) {
    let csql = CString::new(sql).expect("c string");
    let mut affected: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_execute(handle, csql.as_ptr(), &mut affected, &mut message) };
    let text = take_message(message);
    (status, text)
}

fn take_message(message: *mut c_char) -> String {
    if message.is_null() {
        return String::new();
    }
    let value = unsafe { CStr::from_ptr(message) }.to_str().expect("utf-8").to_string();
    unsafe { memless_free_string(message) };
    value
}

#[test]
fn a_transaction_reads_its_own_writes_and_commits_once_to_disk() {
    let (handle, dir) = load_copy("start.yaml");
    let file = dir.join("start.yaml");
    let before = std::fs::read_to_string(&file).expect("read");
    assert_eq!(run_execute(handle, "BEGIN"), (MemlessStatus::Ok, 0));
    assert_eq!(run_execute(handle, "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"), (MemlessStatus::Ok, 1));
    assert_eq!(name_of(handle, "01H7B2"), "Zoe");
    assert_eq!(std::fs::read_to_string(&file).expect("read"), before);
    assert_eq!(run_execute(handle, "COMMIT"), (MemlessStatus::Ok, 0));
    let after = std::fs::read_to_string(&file).expect("read");
    assert_ne!(after, before);
    assert!(after.contains("Zoe"));
    memless_release(handle);
}

#[test]
fn a_rollback_leaves_the_file_untouched_and_restores_the_state() {
    let (handle, dir) = load_copy("start.yaml");
    let file = dir.join("start.yaml");
    let before = std::fs::read_to_string(&file).expect("read");
    assert_eq!(run_execute(handle, "BEGIN"), (MemlessStatus::Ok, 0));
    assert_eq!(run_execute(handle, "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'"), (MemlessStatus::Ok, 1));
    assert_eq!(run_execute(handle, "ROLLBACK"), (MemlessStatus::Ok, 0));
    assert_eq!(std::fs::read_to_string(&file).expect("read"), before);
    assert_eq!(name_of(handle, "01H7B2"), "Ada");
    memless_release(handle);
}

#[test]
fn a_second_begin_is_refused_naming_the_rule() {
    let (handle, _dir) = load_copy("start.yaml");
    assert_eq!(run_execute(handle, "BEGIN"), (MemlessStatus::Ok, 0));
    assert_eq!(execute_message(handle, "BEGIN"), (MemlessStatus::Refused, "a transaction is already open".to_string()));
    memless_release(handle);
}

#[test]
fn a_commit_without_a_transaction_is_refused_naming_the_rule() {
    let (handle, _dir) = load_copy("start.yaml");
    assert_eq!(execute_message(handle, "COMMIT"), (MemlessStatus::Refused, "no open transaction".to_string()));
    memless_release(handle);
}

#[test]
fn a_rollback_without_a_transaction_is_refused_naming_the_rule() {
    let (handle, _dir) = load_copy("start.yaml");
    assert_eq!(execute_message(handle, "ROLLBACK"), (MemlessStatus::Refused, "no open transaction".to_string()));
    memless_release(handle);
}

#[test]
fn a_failed_validation_is_refused_and_closes_the_transaction() {
    let (handle, dir) = load_copy("start.yaml");
    let file = dir.join("start.yaml");
    let before = std::fs::read_to_string(&file).expect("read");
    assert_eq!(run_execute(handle, "BEGIN"), (MemlessStatus::Ok, 0));
    assert_eq!(run_execute(handle, "INSERT INTO users (id, name) VALUES ('01H7B2', 'Dup')"), (MemlessStatus::Ok, 1));
    assert_eq!(run_execute(handle, "COMMIT").0, MemlessStatus::Refused);
    assert_eq!(std::fs::read_to_string(&file).expect("read"), before);
    assert_eq!(run_execute(handle, "BEGIN"), (MemlessStatus::Ok, 0));
    memless_release(handle);
}

#[test]
fn executes_a_write_and_counts_the_rows() {
    let (handle, _dir) = load_copy("start.yaml");
    let (status, affected) = run_execute(handle, "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
    assert_eq!(status, MemlessStatus::Ok);
    assert_eq!(affected, 1);
    memless_release(handle);
}

#[test]
fn a_select_passed_to_execute_is_refused() {
    let (handle, _dir) = load_copy("start.yaml");
    let (status, _) = run_execute(handle, "SELECT * FROM users");
    assert_eq!(status, MemlessStatus::Refused);
    memless_release(handle);
}

#[test]
fn an_unknown_handle_execute_is_invalid_argument() {
    let (status, _) = run_execute(987_654, "DELETE FROM users");
    assert_eq!(status, MemlessStatus::InvalidArgument);
}

#[test]
fn a_null_out_affected_drops_the_count_but_still_writes() {
    let (handle, _dir) = load_copy("start.yaml");
    let csql = CString::new("DELETE FROM wallets").expect("c string");
    let status = unsafe { memless_execute(handle, csql.as_ptr(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::Ok);
    let (qstatus, result) = run_query(handle, "SELECT COUNT(*) FROM wallets");
    assert_eq!(qstatus, MemlessStatus::Ok);
    let mut integer: i64 = -1;
    unsafe { memless_result_cell(result, 0, 0, &mut integer, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(integer, 0);
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn a_null_sql_execute_is_invalid_argument() {
    let (handle, _dir) = load_copy("start.yaml");
    let mut affected: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_execute(handle, ptr::null(), &mut affected, &mut message) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    if !message.is_null() {
        unsafe { memless_free_string(message) };
    }
    memless_release(handle);
}

#[test]
fn a_disk_failure_is_refused_and_leaves_memory_intact() {
    let (handle, dir) = load_copy("start.yaml");
    std::fs::remove_dir_all(&dir).expect("remove workdir");
    let (status, _) = run_execute(handle, "UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");
    assert_eq!(status, MemlessStatus::Refused);
    let (qstatus, result) = run_query(handle, "SELECT name FROM users WHERE id = '01H7B2'");
    assert_eq!(qstatus, MemlessStatus::Ok);
    let mut text: *const c_char = ptr::null();
    unsafe { memless_result_cell(result, 0, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), &mut text) };
    let value = unsafe { CStr::from_ptr(text) }.to_str().expect("utf-8").to_string();
    assert_eq!(value, "Ada");
    memless_result_release(result);
    memless_release(handle);
}

fn load_handle(name: &str) -> u64 {
    let path = fixture(name);
    let mut handle: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_load(path.as_ptr(), &mut handle, &mut message) };
    assert_eq!(status, MemlessStatus::Ok);
    handle
}

fn run_query(handle: u64, sql: &str) -> (MemlessStatus, u64) {
    let csql = CString::new(sql).expect("c string");
    let mut result: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_query(handle, csql.as_ptr(), &mut result, &mut message) };
    if !message.is_null() {
        unsafe { memless_free_string(message) };
    }
    (status, result)
}

#[test]
fn queries_and_reads_an_integer_cell() {
    let handle = load_handle("start.yaml");
    let (status, result) = run_query(handle, "SELECT COUNT(*) FROM users");
    assert_eq!(status, MemlessStatus::Ok);
    assert_eq!(memless_result_column_count(result), 1);
    assert_eq!(memless_result_row_count(result), 1);
    let name = unsafe { CStr::from_ptr(memless_result_column(result, 0)) };
    assert_eq!(name.to_str().expect("utf-8"), "COUNT(*)");
    let mut integer: i64 = 0;
    let kind = unsafe {
        memless_result_cell(result, 0, 0, &mut integer, ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
    };
    assert_eq!(kind, MemlessKind::Integer);
    assert_eq!(integer, 2);
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn reads_a_text_cell_borrowed_until_release() {
    let handle = load_handle("start.yaml");
    let (status, result) = run_query(handle, "SELECT name FROM users");
    assert_eq!(status, MemlessStatus::Ok);
    assert_eq!(memless_result_row_count(result), 2);
    let mut text: *const c_char = ptr::null();
    let kind = unsafe {
        memless_result_cell(result, 0, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), &mut text)
    };
    assert_eq!(kind, MemlessKind::Text);
    let value = unsafe { CStr::from_ptr(text) };
    assert_eq!(value.to_str().expect("utf-8"), "Ada");
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn refuses_an_unknown_table_query_naming_the_rule() {
    let handle = load_handle("start.yaml");
    let csql = CString::new("SELECT * FROM ghosts").expect("c string");
    let mut result: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_query(handle, csql.as_ptr(), &mut result, &mut message) };
    assert_eq!(status, MemlessStatus::Refused);
    let text = unsafe { CStr::from_ptr(message) }.to_str().expect("utf-8").to_string();
    assert_eq!(text, "no table \"ghosts\"");
    unsafe { memless_free_string(message) };
    memless_release(handle);
}

#[test]
fn an_unknown_handle_query_is_invalid_argument() {
    let (status, _) = run_query(987_654, "SELECT * FROM users");
    assert_eq!(status, MemlessStatus::InvalidArgument);
}

#[test]
fn an_unknown_result_reads_as_empty() {
    assert_eq!(memless_result_column_count(555), 0);
    assert_eq!(memless_result_row_count(555), 0);
    assert!(memless_result_column(555, 0).is_null());
    let kind = unsafe {
        memless_result_cell(555, 0, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
    };
    assert_eq!(kind, MemlessKind::Absent);
    memless_result_release(555);
}

#[test]
fn releasing_a_result_twice_is_ignored() {
    let handle = load_handle("start.yaml");
    let (_, result) = run_query(handle, "SELECT COUNT(*) FROM users");
    memless_result_release(result);
    memless_result_release(result);
    memless_release(handle);
}

fn kind_at(result: u64, row: u64, column: u64) -> (MemlessKind, i64, f64, i32, *const c_char) {
    let mut integer: i64 = -1;
    let mut decimal: f64 = -1.0;
    let mut boolean: i32 = -1;
    let mut text: *const c_char = ptr::null();
    let kind = unsafe {
        memless_result_cell(result, row, column, &mut integer, &mut decimal, &mut boolean, &mut text)
    };
    (kind, integer, decimal, boolean, text)
}

#[test]
fn reads_decimal_boolean_and_absent_cells() {
    let handle = load_handle("kinds.yaml");
    let (status, result) = run_query(handle, "SELECT ratio, active, label FROM things");
    assert_eq!(status, MemlessStatus::Ok);
    assert_eq!(memless_result_row_count(result), 2);
    let (ratio_kind, _, ratio, ..) = kind_at(result, 0, 0);
    assert_eq!(ratio_kind, MemlessKind::Decimal);
    assert_eq!(ratio, 1.5);
    let (active_kind, .., active, _) = kind_at(result, 0, 1);
    assert_eq!(active_kind, MemlessKind::Boolean);
    assert_eq!(active, 1);
    let (label_kind, integer, ..) = kind_at(result, 1, 2);
    assert_eq!(label_kind, MemlessKind::Absent);
    assert_eq!(integer, -1);
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn out_of_range_cell_is_absent_and_writes_nothing() {
    let handle = load_handle("start.yaml");
    let (_, result) = run_query(handle, "SELECT COUNT(*) FROM users");
    let (kind, integer, ..) = kind_at(result, 99, 99);
    assert_eq!(kind, MemlessKind::Absent);
    assert_eq!(integer, -1);
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn out_of_range_column_is_null() {
    let handle = load_handle("start.yaml");
    let (_, result) = run_query(handle, "SELECT COUNT(*) FROM users");
    assert!(memless_result_column(result, 99).is_null());
    memless_result_release(result);
    memless_release(handle);
}

#[test]
fn a_null_sql_is_invalid_argument() {
    let handle = load_handle("start.yaml");
    let mut result: u64 = 0;
    let mut message: *mut c_char = ptr::null_mut();
    let status = unsafe { memless_query(handle, ptr::null(), &mut result, &mut message) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    if !message.is_null() {
        unsafe { memless_free_string(message) };
    }
    memless_release(handle);
}

#[test]
fn a_null_out_result_is_invalid_argument() {
    let handle = load_handle("start.yaml");
    let csql = CString::new("SELECT * FROM users").expect("c string");
    let status = unsafe { memless_query(handle, csql.as_ptr(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::InvalidArgument);
    memless_release(handle);
}

#[test]
fn returns_the_status_alone_when_out_message_is_null_on_refused_query() {
    let handle = load_handle("start.yaml");
    let csql = CString::new("SELECT * FROM ghosts").expect("c string");
    let mut result: u64 = 0;
    let status = unsafe { memless_query(handle, csql.as_ptr(), &mut result, ptr::null_mut()) };
    assert_eq!(status, MemlessStatus::Refused);
    memless_release(handle);
}

#[test]
fn a_result_outlives_the_released_instance() {
    let handle = load_handle("start.yaml");
    let (_, result) = run_query(handle, "SELECT name FROM users");
    memless_release(handle);
    assert_eq!(memless_result_row_count(result), 2);
    let (kind, .., text) = kind_at(result, 1, 0);
    assert_eq!(kind, MemlessKind::Text);
    let value = unsafe { CStr::from_ptr(text) }.to_str().expect("utf-8");
    assert_eq!(value, "Grace");
    memless_result_release(result);
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
