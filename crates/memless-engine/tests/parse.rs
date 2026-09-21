mod common;

use common::{read_path, temp_file};
use memless_domain::document::{RawKey, RawNode, RawStyle};
use memless_engine::read;

fn root_of(name: &str, content: &str) -> RawNode {
    let path = temp_file(name, content);
    read(&read_path(&path)).expect("document").root
}

fn styles_of(entries: &[(RawKey, RawNode)]) -> Vec<RawStyle> {
    entries
        .iter()
        .map(|(_, value)| match value {
            RawNode::Scalar(scalar) => scalar.style.clone(),
            _ => panic!("expected a scalar"),
        })
        .collect()
}

#[test]
fn preserves_key_order_in_a_mapping() {
    let root = root_of("order.yaml", "b: 1\na: 2\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    let keys: Vec<RawKey> = entries.iter().map(|(key, _)| key.clone()).collect();
    assert_eq!(keys, vec![RawKey::Text("b".to_string()), RawKey::Text("a".to_string())]);
}

#[test]
fn preserves_duplicated_keys() {
    let root = root_of("dup.yaml", "a: 1\na: 2\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert_eq!(entries.len(), 2);
}

#[test]
fn preserves_a_non_text_key() {
    let root = root_of("nontext.yaml", "1: one\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert!(matches!(entries[0].0, RawKey::NonText { .. }));
}

#[test]
fn classifies_a_boolean_key_as_non_text() {
    let root = root_of("boolkey.yaml", "true: yes\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert!(matches!(entries[0].0, RawKey::NonText { .. }));
}

#[test]
fn classifies_a_structural_key_as_non_text() {
    let root = root_of("structkey.yaml", "? [a, b]\n: v\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert!(matches!(entries[0].0, RawKey::NonText { .. }));
}

#[test]
fn tags_a_block_and_tagged_scalar_as_explicit_text() {
    let root = root_of("explicit.yaml", "a: |\n  block\nb: !!str 1\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert_eq!(styles_of(entries), vec![RawStyle::ExplicitText, RawStyle::ExplicitText]);
}

#[test]
fn preserves_a_nested_value() {
    let root = root_of("nested.yaml", "a: [1, 2]\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert!(matches!(entries[0].1, RawNode::Sequence(_)));
}

#[test]
fn tags_each_scalar_with_its_style() {
    let root = root_of("style.yaml", "a: 1\nb: \"1\"\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert_eq!(styles_of(entries), vec![RawStyle::Plain, RawStyle::ExplicitText]);
}

#[test]
fn guesses_no_type_leaving_each_scalar_a_raw_lexeme() {
    let root = root_of("raw.yaml", "a: 1\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    match &entries[0].1 {
        RawNode::Scalar(scalar) => assert_eq!(scalar.lexeme, "1"),
        _ => panic!("expected a scalar"),
    }
}

#[test]
fn maps_empty_and_explicit_null_values_to_null() {
    let root = root_of("null.yaml", "a:\nb: null\nc: ~\n");
    let RawNode::Mapping(entries) = &root else {
        panic!("expected a mapping");
    };
    assert!(matches!(entries[0].1, RawNode::Null));
    assert!(matches!(entries[1].1, RawNode::Null));
    assert!(matches!(entries[2].1, RawNode::Null));
}
