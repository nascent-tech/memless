use std::ffi::CString;

pub(crate) fn c_text(text: &str) -> CString {
    CString::new(text.replace('\0', "\u{FFFD}")).unwrap_or_default()
}
