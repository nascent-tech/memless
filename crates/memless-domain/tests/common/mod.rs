#![allow(dead_code)]

use memless_domain::document::{RawDocument, RawKey, RawNode, RawScalar, RawStyle};

pub fn plain(lexeme: &str) -> RawNode {
    RawNode::Scalar(RawScalar { lexeme: lexeme.to_string(), style: RawStyle::Plain })
}

pub fn quoted(lexeme: &str) -> RawNode {
    RawNode::Scalar(RawScalar { lexeme: lexeme.to_string(), style: RawStyle::ExplicitText })
}

pub fn field(name: &str, value: RawNode) -> (RawKey, RawNode) {
    (RawKey::Text(name.to_string()), value)
}

pub fn nontext_field(rendered: &str, value: RawNode) -> (RawKey, RawNode) {
    (RawKey::NonText { rendered: rendered.to_string() }, value)
}

pub fn record(fields: Vec<(RawKey, RawNode)>) -> RawNode {
    RawNode::Mapping(fields)
}

pub fn rows(items: Vec<RawNode>) -> RawNode {
    RawNode::Sequence(items)
}

pub fn document(tables: Vec<(RawKey, RawNode)>) -> RawDocument {
    RawDocument { source: "test.yaml".to_string(), root: RawNode::Mapping(tables) }
}
