use super::scalar::render_scalar;
use memless_domain::document::RawNode;

pub(super) fn value_text(value: &RawNode) -> String {
    match value {
        RawNode::Scalar(scalar) => render_scalar(scalar),
        _ => String::new(),
    }
}
