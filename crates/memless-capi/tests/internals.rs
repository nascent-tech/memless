use std::ffi::CStr;

use memless_capi::{guard, memless_free_string, own_message};

#[test]
fn guard_returns_the_fallback_when_the_body_panics() {
    let value = guard(7, || panic!("boom"));
    assert_eq!(value, 7);
}

#[test]
fn guard_returns_the_body_result_when_it_does_not_panic() {
    assert_eq!(guard(0, || 42), 42);
}

#[test]
fn own_message_replaces_an_interior_nul_with_the_replacement_char() {
    let message = own_message("a\0b");
    let text = unsafe { CStr::from_ptr(message) }.to_str().expect("utf-8").to_string();
    assert_eq!(text, "a\u{FFFD}b");
    unsafe { memless_free_string(message) };
}
