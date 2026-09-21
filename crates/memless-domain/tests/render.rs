mod common;

use common::{document, field, plain, quoted, record, rows};
use memless_domain::Base;

fn reload(base: &Base) -> Base {
    Base::load(base.document()).unwrap()
}

fn shop() -> Base {
    Base::load(document(vec![
        field(
            "users",
            rows(vec![
                record(vec![field("id", plain("1")), field("role", quoted("ADMIN"))]),
                record(vec![field("id", plain("2")), field("role", quoted("USER"))]),
            ]),
        ),
        field(
            "wallets",
            rows(vec![
                record(vec![field("id", quoted("w1")), field("user_id", plain("1")), field("balance", plain("100"))]),
                record(vec![field("id", quoted("w2")), field("user_id", plain("2")), field("balance", plain("50"))]),
            ]),
        ),
    ]))
    .unwrap()
}

#[test]
fn round_trip_preserves_a_multi_table_base() {
    let base = shop();
    assert!(reload(&base) == base);
}

#[test]
fn round_trip_keeps_text_that_would_be_guessed_as_a_number() {
    let base = Base::load(document(vec![field(
        "codes",
        rows(vec![record(vec![field("id", plain("1")), field("label", quoted("5"))])]),
    )]))
    .unwrap();
    assert!(reload(&base) == base);
}

#[test]
fn round_trip_keeps_text_that_would_be_guessed_as_a_boolean() {
    let base = Base::load(document(vec![field(
        "flags",
        rows(vec![record(vec![field("id", plain("1")), field("label", quoted("true"))])]),
    )]))
    .unwrap();
    assert!(reload(&base) == base);
}

#[test]
fn round_trip_keeps_a_decimal() {
    let base = Base::load(document(vec![field(
        "sums",
        rows(vec![record(vec![field("id", plain("1")), field("total", plain("1.5"))])]),
    )]))
    .unwrap();
    assert!(reload(&base) == base);
}

#[test]
fn round_trip_preserves_a_row_missing_a_column() {
    let base = Base::load(document(vec![field(
        "users",
        rows(vec![
            record(vec![field("id", plain("1")), field("role", quoted("A"))]),
            record(vec![field("id", plain("2"))]),
        ]),
    )]))
    .unwrap();
    assert!(reload(&base) == base);
}

#[test]
fn round_trip_preserves_an_empty_table() {
    let base = Base::load(document(vec![field("logs", rows(vec![]))])).unwrap();
    assert!(reload(&base) == base);
}
