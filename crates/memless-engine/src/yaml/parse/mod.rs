use memless_domain::document::RawNode;
use memless_domain::refusal::TextPosition;
use serde_saphyr::granit_parser::Parser;

use builder::Builder;
use options::saturate_options;
use scan::scan_failure;
use step::step;

mod alias;
mod attach;
mod builder;
mod classify;
mod collections;
mod document;
mod frame;
mod mapping_attach;
mod node_key;
mod options;
mod place;
mod scalar_kind;
mod scalar_node;
mod scan;
mod step;

pub(crate) struct ParseFailure {
    pub at: Option<TextPosition>,
}

pub(crate) fn parse(source: &str) -> Result<RawNode, ParseFailure> {
    let mut builder = Builder::default();
    for item in Parser::new_from_str_with_options(source, saturate_options()) {
        let (event, _span) = item.map_err(scan_failure)?;
        step(&mut builder, event)?;
    }
    builder.root.ok_or(ParseFailure { at: None })
}
