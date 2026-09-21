use super::frame::Frame;
use super::node_key::node_to_key;
use memless_domain::document::RawNode;

pub(super) fn attach_to_mapping(frame: &mut Frame, node: RawNode) {
    let Frame::Mapping { entries, pending, .. } = frame else {
        return;
    };
    match pending.take() {
        None => *pending = Some(node_to_key(&node)),
        Some(key) => entries.push((key, node)),
    }
}
