mod common;

use common::{document, field, nontext_field, plain, quoted, record, rows};
use memless_domain::document::{RawDocument, RawNode};
use memless_domain::Base;

fn refusal_of(document: RawDocument) -> String {
    Base::load(document).expect_err("expected a refusal").to_string()
}

#[test]
fn refuses_a_root_scalar_as_no_table_declared() {
    let scalar_root = RawDocument { source: "test.yaml".to_string(), root: plain("hello") };
    assert_eq!(refusal_of(scalar_root), "no table declared in \"test.yaml\"");
}

#[test]
fn refuses_an_empty_mapping_as_no_table_declared() {
    let empty = RawDocument { source: "test.yaml".to_string(), root: RawNode::Mapping(Vec::new()) };
    assert_eq!(refusal_of(empty), "no table declared in \"test.yaml\"");
}

#[test]
fn refuses_a_table_that_is_not_a_row_list() {
    let document = document(vec![field("users", plain("x"))]);
    assert_eq!(refusal_of(document), "table \"users\" is not a list of rows");
}

#[test]
fn refuses_a_null_table_as_not_a_row_list() {
    let document = document(vec![field("users", RawNode::Null)]);
    assert_eq!(refusal_of(document), "table \"users\" is not a list of rows");
}

#[test]
fn refuses_a_row_that_is_not_a_field_set() {
    let document = document(vec![field("users", rows(vec![plain("x")]))]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" is not a field set");
}

#[test]
fn refuses_a_nested_field_value_naming_the_row_by_its_id() {
    let row = record(vec![field("id", plain("7")), field("tags", rows(vec![plain("a")]))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "nested value in \"tags\" of row 7 in \"users\"");
}

#[test]
fn names_a_row_without_an_id_by_its_position() {
    let row = record(vec![field("name", quoted("a")), field("tags", rows(vec![plain("a")]))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "nested value in \"tags\" of row 1 in \"users\"");
}

#[test]
fn refuses_a_root_sequence_as_no_table_declared() {
    let sequence_root = RawDocument { source: "test.yaml".to_string(), root: rows(vec![plain("x")]) };
    assert_eq!(refusal_of(sequence_root), "no table declared in \"test.yaml\"");
}

#[test]
fn refuses_a_duplicate_column_key_even_when_the_first_value_is_null() {
    let row = record(vec![field("id", plain("7")), field("name", RawNode::Null), field("name", quoted("x"))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "\"name\" duplicated in row 7 of \"users\"");
}

#[test]
fn refuses_a_duplicate_id_key_even_when_the_first_id_is_null() {
    let row = record(vec![field("id", RawNode::Null), field("id", plain("5"))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "\"id\" duplicated in row 5 of \"users\"");
}

#[test]
fn refuses_a_duplicated_table_key() {
    let document = document(vec![field("users", rows(vec![])), field("users", rows(vec![]))]);
    assert_eq!(refusal_of(document), "duplicate table key \"users\"");
}

#[test]
fn refuses_a_duplicated_column_key_in_one_row() {
    let row = record(vec![field("id", plain("1")), field("name", quoted("a")), field("name", quoted("b"))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "\"name\" duplicated in row 1 of \"users\"");
}

#[test]
fn refuses_a_non_text_table_key() {
    let document = document(vec![nontext_field("5", rows(vec![]))]);
    assert_eq!(refusal_of(document), "non-text key 5 at position 1");
}

#[test]
fn refuses_a_non_text_column_key_naming_its_table() {
    let row = record(vec![field("id", plain("1")), nontext_field("7", quoted("a"))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert_eq!(refusal_of(document), "non-text key 7 at position 1 in table \"users\"");
}

#[test]
fn accepts_an_empty_table_as_a_table_with_no_rows() {
    let document = document(vec![field("users", rows(vec![]))]);
    assert!(Base::load(document).is_ok());
}
