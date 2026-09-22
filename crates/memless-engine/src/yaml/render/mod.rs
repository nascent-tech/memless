use memless_domain::document::{RawDocument, RawNode};
use table::table_block;

mod blank_control;
mod escape;
mod escape_char;
mod field;
mod flow_null;
mod indicator;
mod key;
mod key_guess;
mod quote;
mod row;
mod scalar;
mod table;
mod value;

pub fn render(document: &RawDocument) -> String {
    let RawNode::Mapping(entries) = &document.root else {
        return String::new();
    };
    entries.iter().map(table_block).collect()
}
