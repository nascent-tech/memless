use super::scalar_kind::{is_null, raw_style};
use memless_domain::document::{RawNode, RawScalar};
use serde_saphyr::granit_parser::{ScalarStyle, Tag};

pub(super) fn scalar_node(value: &str, style: ScalarStyle, tag: Option<&Tag>) -> RawNode {
    if is_null(value, style, tag) {
        return RawNode::Null;
    }
    RawNode::Scalar(RawScalar { lexeme: value.to_string(), style: raw_style(style, tag) })
}
