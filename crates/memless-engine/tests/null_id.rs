mod common;

use common::{read_path, temp_file};
use memless_engine::{load, read};

fn outcome(name: &str, content: &str) -> Option<String> {
    let path = temp_file(name, content);
    match load(read, &read_path(&path)) {
        Ok(_) => None,
        Err(refusal) => Some(refusal.to_string()),
    }
}

#[test]
fn treats_every_null_id_spelling_as_a_missing_id() {
    let cases = [
        ("id_null.yaml", "users:\n  - id: null\n"),
        ("id_tilde.yaml", "users:\n  - id: ~\n"),
        ("id_empty.yaml", "users:\n  - id:\n"),
        ("id_upper.yaml", "users:\n  - id: NULL\n"),
    ];
    for (name, content) in cases {
        assert_eq!(outcome(name, content).as_deref(), Some("row 1 in \"users\" has no id"));
    }
}

#[test]
fn accepts_a_quoted_null_as_a_text_id() {
    assert_eq!(outcome("id_quoted_null.yaml", "users:\n  - id: \"null\"\n"), None);
}
