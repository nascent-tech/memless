use std::ffi::CString;

use super::cell::Cell;

pub(crate) struct Prepared {
    pub columns: Vec<CString>,
    pub rows: Vec<Vec<Cell>>,
}
