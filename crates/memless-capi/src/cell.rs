use std::ffi::CString;

use memless_engine::Scalar;

use super::c_text::c_text;

pub(crate) enum Cell {
    Absent,
    Text(CString),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
}

pub(crate) fn cell_of(value: &Option<Scalar>) -> Cell {
    match value {
        None => Cell::Absent,
        Some(Scalar::Text(text)) => Cell::Text(c_text(text)),
        Some(Scalar::Integer(number)) => Cell::Integer(*number),
        Some(Scalar::Decimal(number)) => Cell::Decimal(*number),
        Some(Scalar::Boolean(flag)) => Cell::Boolean(*flag),
    }
}
