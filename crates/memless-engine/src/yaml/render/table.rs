use super::key::render_key;
use super::row::row_block;
use memless_domain::document::{RawKey, RawNode};

pub(super) fn table_block(entry: &(RawKey, RawNode)) -> String {
    let (key, value) = entry;
    let RawNode::Sequence(rows) = value else {
        return String::new();
    };
    if rows.is_empty() {
        return format!("{}: []\n", render_key(key));
    }
    let body: String = rows.iter().map(row_block).collect();
    format!("{}:\n{body}", render_key(key))
}
