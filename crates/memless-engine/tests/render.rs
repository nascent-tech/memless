mod common;

use std::sync::atomic::{AtomicU64, Ordering};

use common::{read_path, temp_file};
use memless_engine::{load, read, render, Base};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn base_of(prefix: &str, yaml: &str) -> Base {
    let name = format!("{prefix}-{}.yaml", COUNTER.fetch_add(1, Ordering::Relaxed));
    let path = temp_file(&name, yaml);
    load(read, &read_path(&path)).unwrap().base
}

fn reload(prefix: &str, yaml: &str) -> Base {
    base_of(prefix, &render(&base_of(prefix, yaml).document()))
}

#[test]
fn renders_a_canonical_file_byte_for_byte() {
    let yaml = "\
users:
  - id: 1
    role: ADMIN
  - id: 2
    role: USER
wallets:
  - id: w1
    user_id: 1
    balance: 100
  - id: w2
    user_id: 2
    balance: 50
";
    assert_eq!(render(&base_of("render-shop", yaml).document()), yaml);
}

#[test]
fn quotes_text_that_would_be_guessed_as_a_number() {
    let yaml = "codes:\n  - id: 1\n    label: \"5\"\n";
    assert_eq!(render(&base_of("render-codes", yaml).document()), yaml);
}

#[test]
fn quotes_text_that_would_be_guessed_as_a_boolean() {
    let yaml = "flags:\n  - id: 1\n    label: \"true\"\n";
    assert_eq!(render(&base_of("render-flags", yaml).document()), yaml);
}

#[test]
fn renders_an_empty_table_inline() {
    let yaml = "logs: []\n";
    assert_eq!(render(&base_of("render-empty", yaml).document()), yaml);
}

#[test]
fn omits_a_column_a_row_does_not_carry() {
    let yaml = "users:\n  - id: 1\n    role: ADMIN\n  - id: 2\n";
    assert_eq!(render(&base_of("render-sparse", yaml).document()), yaml);
}

fn round_trips(prefix: &str, yaml: &str) {
    assert!(reload(prefix, yaml) == base_of(prefix, yaml));
}

#[test]
fn quotes_a_value_ending_in_a_colon() {
    round_trips("render-colon", "t:\n  - id: 1\n    v: \"Total:\"\n");
}

#[test]
fn quotes_a_key_that_would_be_guessed_as_a_number() {
    round_trips("render-numkey", "t:\n  - id: 1\n    \"5\": x\n");
}

#[test]
fn quotes_a_key_that_would_be_guessed_as_a_boolean() {
    round_trips("render-boolkey", "t:\n  - id: 1\n    \"true\": x\n");
}

#[test]
fn round_trips_a_boundary_corpus_of_values() {
    let corpus = [
        "\"\"", "\" x\"", "\"x \"", "\"- x\"", "\"? x\"", "\"#a\"", "\"a: b\"", "\"a #\"", "\"~\"",
        "\"null\"", "\"NULL\"", "\"Total:\"", "\"a:b:\"", "\"@handle\"", "\"a\\nb\"", "\"true\"",
    ];
    for (index, value) in corpus.iter().enumerate() {
        let yaml = format!("t:\n  - id: 1\n    v: {value}\n");
        round_trips(&format!("render-corpus-{index}"), &yaml);
    }
}

#[test]
fn round_trips_awkward_numbers_and_stays_canonical() {
    round_trips("render-bignum", "t:\n  - id: 1\n    v: 1000000000000000000000.0\n");
    round_trips("render-neg", "t:\n  - id: 1\n    v: -5\n");
    round_trips("render-dec", "t:\n  - id: 1\n    v: 1.5\n");
}

#[test]
fn a_reload_of_the_render_equals_the_base() {
    let yaml = "\
users:
  - id: 1
    role: ADMIN
wallets:
  - id: w1
    user_id: 1
    balance: 100
";
    assert!(reload("render-trip", yaml) == base_of("render-trip", yaml));
}
