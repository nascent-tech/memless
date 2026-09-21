use super::RawNode;

pub(super) fn push_children(node: &mut RawNode, pending: &mut Vec<RawNode>) {
    match node {
        RawNode::Sequence(items) => pending.append(items),
        RawNode::Mapping(entries) => pending.extend(entries.drain(..).map(|(_, value)| value)),
        _ => {}
    }
}
