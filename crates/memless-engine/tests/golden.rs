use memless_engine::{load, read};

fn fixture(name: &str) -> String {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../harness/parity/fixtures");
    format!("{base}/{name}")
}

fn refusal(name: &str) -> String {
    match load(read, &fixture(name)) {
        Ok(_) => panic!("expected a refusal for {name}"),
        Err(refusal) => refusal.to_string(),
    }
}

#[test]
fn loads_the_start_file() {
    assert!(load(read, &fixture("start.yaml")).is_ok());
}

#[test]
fn loads_mixed_ids_as_two_rows() {
    assert!(load(read, &fixture("loads-mixed-id.yaml")).is_ok());
}

#[test]
fn loads_bounded_anchors_without_a_numeric_refusal() {
    assert!(load(read, &fixture("loads-anchors.yaml")).is_ok());
}

#[test]
fn refuses_an_empty_file() {
    let expected = format!("file at path {:?} is empty", fixture("empty.yaml"));
    assert_eq!(refusal("empty.yaml"), expected);
}

#[test]
fn refuses_invalid_yaml_naming_the_path() {
    let prefix = format!("invalid YAML in {:?}", fixture("invalid-yaml.yaml"));
    assert!(refusal("invalid-yaml.yaml").starts_with(&prefix));
}

#[test]
fn refuses_a_file_with_no_table() {
    let expected = format!("no table declared in {:?}", fixture("no-table.yaml"));
    assert_eq!(refusal("no-table.yaml"), expected);
}

#[test]
fn refuses_each_shape_and_coherence_fixture_with_its_exact_text() {
    assert_eq!(refusal("table-not-list.yaml"), "table \"users\" is not a list of rows");
    assert_eq!(refusal("row-not-map.yaml"), "row 1 in \"users\" is not a field set");
    assert_eq!(refusal("nested-value.yaml"), "nested value in \"tags\" of row 1 in \"users\"");
    assert_eq!(refusal("dup-table-key.yaml"), "duplicate table key \"users\"");
    assert_eq!(refusal("dup-column-key.yaml"), "\"name\" duplicated in row 1 of \"users\"");
    assert_eq!(refusal("non-text-key.yaml"), "non-text key 1 at position 1");
    assert_eq!(refusal("missing-id.yaml"), "row 1 in \"users\" has no id");
    assert_eq!(refusal("id-decimal.yaml"), "row 1 in \"users\" has a non-text non-integer id: 1.5");
    assert_eq!(refusal("id-boolean.yaml"), "row 1 in \"users\" has a non-text non-integer id: true");
    assert_eq!(refusal("dup-id.yaml"), "duplicate id 1 in \"users\" at rows 1 and 2");
}

#[test]
fn refuses_a_broken_relation() {
    let expected = "broken relation \"user_id\" of row 1 in \"orders\": no row 9 in \"users\"";
    assert_eq!(refusal("broken-relation.yaml"), expected);
}

#[test]
fn refuses_a_self_relation() {
    let expected = "broken relation \"tag_id\" of row 1 in \"tags\": no row 9 in \"tags\"";
    assert_eq!(refusal("self-relation.yaml"), expected);
}

#[test]
fn leaves_the_neighbouring_residue_intact_after_loading() {
    let residue = fixture(".start.yaml.memless-tmp");
    let before = std::fs::read(&residue).expect("read residue");
    assert!(load(read, &fixture("start.yaml")).is_ok());
    let after = std::fs::read(&residue).expect("read residue");
    assert_eq!(before, after);
}
