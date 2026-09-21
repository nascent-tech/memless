use super::alias::on_alias;
use super::builder::Builder;
use super::collections::{on_mapping_end, on_mapping_start, on_sequence_end, on_sequence_start};
use super::document::on_document_start;
use super::place::place;
use super::scalar_node::scalar_node;
use super::ParseFailure;
use serde_saphyr::granit_parser::Event;

pub(super) fn step(builder: &mut Builder, event: Event) -> Result<(), ParseFailure> {
    match event {
        Event::DocumentStart(_, _) => on_document_start(builder),
        Event::Scalar(text, style, anchor, tag) => place(builder, anchor, scalar_node(&text, style, tag.as_deref())),
        Event::SequenceStart(_, anchor, _) => on_sequence_start(builder, anchor),
        Event::SequenceEnd => on_sequence_end(builder),
        Event::MappingStart(_, anchor, _) => on_mapping_start(builder, anchor),
        Event::MappingEnd => on_mapping_end(builder),
        Event::Alias(id) => on_alias(builder, id),
        _ => Ok(()),
    }
}
