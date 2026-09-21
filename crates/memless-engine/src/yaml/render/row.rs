use super::field::field_line;
use memless_domain::document::RawNode;

pub(super) fn row_block(row: &RawNode) -> String {
    let RawNode::Mapping(fields) = row else {
        return String::new();
    };
    fields.iter().enumerate().map(|(index, field)| field_line(index, field)).collect()
}
