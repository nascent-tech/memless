use super::children::push_children;
use super::RawNode;

pub(super) fn drain_node(node: &mut RawNode) {
    let mut pending = Vec::new();
    push_children(node, &mut pending);
    while let Some(mut child) = pending.pop() {
        push_children(&mut child, &mut pending);
    }
}
