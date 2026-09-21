mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::document::{RawDocument, RawNode};
use memless_domain::Base;

fn refusal_of(document: RawDocument) -> String {
    Base::load(document).err().expect("expected a refusal").to_string()
}

#[test]
fn accepts_a_relation_that_resolves_to_an_existing_row() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("5"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", plain("5"))])])),
    ]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn refuses_a_relation_pointing_at_no_existing_row() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", plain("9"))])])),
    ]);
    let expected = "broken relation \"user_id\" of row 1 in \"orders\": no row 9 in \"users\"";
    assert_eq!(refusal_of(document), expected);
}

#[test]
fn refuses_a_self_relation_with_no_matching_row() {
    let row = record(vec![field("id", plain("1")), field("tag_id", plain("9"))]);
    let document = document(vec![field("tags", rows(vec![row]))]);
    let expected = "broken relation \"tag_id\" of row 1 in \"tags\": no row 9 in \"tags\"";
    assert_eq!(refusal_of(document), expected);
}

#[test]
fn distinguishes_a_text_relation_value_from_an_integer_id() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("5"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", quoted("5"))])])),
    ]);
    let expected = "broken relation \"user_id\" of row 1 in \"orders\": no row \"5\" in \"users\"";
    assert_eq!(refusal_of(document), expected);
}

#[test]
fn treats_a_column_with_no_target_table_as_ordinary() {
    let row = record(vec![field("id", plain("1")), field("stripe_id", quoted("cus_1"))]);
    let document = document(vec![field("users", rows(vec![row]))]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn ignores_an_absent_relation() {
    let document = document(vec![field("orders", rows(vec![record(vec![field("id", plain("1"))])]))]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn resolves_a_bare_underscore_id_column_to_the_table_named_s() {
    let document = document(vec![
        field("s", rows(vec![record(vec![field("id", plain("1"))])])),
        field("items", rows(vec![record(vec![field("id", plain("1")), field("_id", plain("9"))])])),
    ]);
    let expected = "broken relation \"_id\" of row 1 in \"items\": no row 9 in \"s\"";
    assert_eq!(refusal_of(document), expected);
}

#[test]
fn ignores_a_null_relation_value_even_when_the_target_exists() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", RawNode::Null)])])),
    ]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn refuses_a_relation_whose_value_is_a_decimal() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", plain("1.5"))])])),
    ]);
    let expected = "broken relation \"user_id\" of row 1 in \"orders\": no row 1.5 in \"users\"";
    assert_eq!(refusal_of(document), expected);
}
