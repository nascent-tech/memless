use super::attach::attach;
use super::builder::Builder;
use super::ParseFailure;
use memless_domain::document::RawNode;

pub(super) fn place(builder: &mut Builder, anchor: usize, node: RawNode) -> Result<(), ParseFailure> {
    if anchor != 0 {
        builder.anchors.insert(anchor, node.clone());
    }
    attach(builder, node);
    Ok(())
}
