use super::classify::classify_key;
use memless_domain::document::{RawKey, RawNode};

pub(super) fn node_to_key(node: &RawNode) -> RawKey {
    match node {
        RawNode::Scalar(raw) => classify_key(raw.clone()),
        RawNode::Null => RawKey::NonText { rendered: "null".to_string() },
        RawNode::Sequence(_) => RawKey::NonText { rendered: "sequence".to_string() },
        RawNode::Mapping(_) => RawKey::NonText { rendered: "mapping".to_string() },
    }
}
