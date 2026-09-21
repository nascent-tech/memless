use super::builder::Builder;
use super::frame::Frame;
use super::mapping_attach::attach_to_mapping;
use memless_domain::document::RawNode;

pub(super) fn attach(builder: &mut Builder, node: RawNode) {
    match builder.stack.last_mut() {
        None => builder.root = Some(node),
        Some(Frame::Sequence { items, .. }) => items.push(node),
        Some(frame) => attach_to_mapping(frame, node),
    }
}
