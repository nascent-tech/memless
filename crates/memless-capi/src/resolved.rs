use std::ffi::c_char;

pub(crate) enum Resolved {
    Absent,
    Text(*const c_char),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
}
