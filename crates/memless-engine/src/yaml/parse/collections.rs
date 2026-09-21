use super::builder::Builder;
use super::frame::Frame;
use super::place::place;
use super::ParseFailure;
use memless_domain::document::RawNode;

pub(super) fn on_sequence_start(builder: &mut Builder, anchor: usize) -> Result<(), ParseFailure> {
    builder.stack.push(Frame::Sequence { items: Vec::new(), anchor });
    Ok(())
}

pub(super) fn on_mapping_start(builder: &mut Builder, anchor: usize) -> Result<(), ParseFailure> {
    builder.stack.push(Frame::Mapping { entries: Vec::new(), pending: None, anchor });
    Ok(())
}

pub(super) fn on_sequence_end(builder: &mut Builder) -> Result<(), ParseFailure> {
    match builder.stack.pop() {
        Some(Frame::Sequence { items, anchor }) => place(builder, anchor, RawNode::Sequence(items)),
        _ => Err(ParseFailure { at: None }),
    }
}

pub(super) fn on_mapping_end(builder: &mut Builder) -> Result<(), ParseFailure> {
    match builder.stack.pop() {
        Some(Frame::Mapping { entries, anchor, .. }) => place(builder, anchor, RawNode::Mapping(entries)),
        _ => Err(ParseFailure { at: None }),
    }
}
