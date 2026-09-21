use super::key::render_key;
use super::value::value_text;
use memless_domain::document::{RawKey, RawNode};

pub(super) fn field_line(index: usize, field: &(RawKey, RawNode)) -> String {
    let (key, value) = field;
    let marker = if index == 0 { "  - " } else { "    " };
    format!("{marker}{}: {}\n", render_key(key), value_text(value))
}
