mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::document::{RawDocument, RawNode};
use memless_domain::Base;

fn refusal_of(document: RawDocument) -> String {
    Base::load(document).expect_err("expected a refusal").to_string()
}

fn users_with(row_nodes: Vec<RawNode>) -> RawDocument {
    document(vec![field("users", rows(row_nodes))])
}

#[test]
fn accepts_a_coherent_file() {
    let document = users_with(vec![
        record(vec![field("id", plain("1"))]),
        record(vec![field("id", plain("2"))]),
    ]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn refuses_a_row_without_an_id() {
    let document = users_with(vec![record(vec![field("name", quoted("a"))])]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" has no id");
}

#[test]
fn treats_a_null_id_as_missing() {
    let document = users_with(vec![record(vec![field("id", RawNode::Null)])]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" has no id");
}

#[test]
fn refuses_a_decimal_id() {
    let document = users_with(vec![record(vec![field("id", plain("1.5"))])]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" has a non-text non-integer id: 1.5");
}

#[test]
fn refuses_a_boolean_id() {
    let document = users_with(vec![record(vec![field("id", plain("true"))])]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" has a non-text non-integer id: true");
}

#[test]
fn refuses_two_rows_sharing_an_id() {
    let document = users_with(vec![
        record(vec![field("id", plain("1"))]),
        record(vec![field("id", plain("1"))]),
    ]);
    assert_eq!(refusal_of(document), "duplicate id 1 in \"users\" at rows 1 and 2");
}

#[test]
fn accepts_an_integer_and_a_text_id_of_the_same_digits_as_two_rows() {
    let document = users_with(vec![
        record(vec![field("id", plain("5"))]),
        record(vec![field("id", quoted("5"))]),
    ]);
    assert!(Base::load(document).is_ok());
}

#[test]
fn reports_a_shape_fault_before_a_coherence_fault() {
    let document = document(vec![
        field("bad", plain("x")),
        field("users", rows(vec![record(vec![field("name", quoted("a"))])])),
    ]);
    assert_eq!(refusal_of(document), "table \"bad\" is not a list of rows");
}

#[test]
fn reports_a_missing_id_before_a_duplicate_id() {
    let document = users_with(vec![
        record(vec![field("id", plain("1"))]),
        record(vec![field("id", plain("1"))]),
        record(vec![field("name", quoted("x"))]),
    ]);
    assert_eq!(refusal_of(document), "row 3 in \"users\" has no id");
}

#[test]
fn reports_a_missing_id_in_a_later_table_before_a_duplicate_id_in_an_earlier_one() {
    let document = document(vec![
        field("a", rows(vec![record(vec![field("id", plain("1"))]), record(vec![field("id", plain("1"))])])),
        field("b", rows(vec![record(vec![field("name", quoted("x"))])])),
    ]);
    assert_eq!(refusal_of(document), "row 1 in \"b\" has no id");
}

#[test]
fn reports_a_duplicate_id_before_a_broken_relation() {
    let document = document(vec![
        field("users", rows(vec![record(vec![field("id", plain("1"))]), record(vec![field("id", plain("1"))])])),
        field("orders", rows(vec![record(vec![field("id", plain("1")), field("user_id", plain("9"))])])),
    ]);
    assert_eq!(refusal_of(document), "duplicate id 1 in \"users\" at rows 1 and 2");
}

#[test]
fn reports_the_first_row_when_two_rows_break_the_same_rule() {
    let document = users_with(vec![
        record(vec![field("name", quoted("a"))]),
        record(vec![field("name", quoted("b"))]),
    ]);
    assert_eq!(refusal_of(document), "row 1 in \"users\" has no id");
}

#[test]
fn accepts_a_quoted_text_id() {
    let document = users_with(vec![record(vec![field("id", quoted("5"))])]);
    assert!(Base::load(document).is_ok());
}
