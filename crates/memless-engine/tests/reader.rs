mod common;

use common::{read_path, temp_file};
use memless_domain::refusal::SourceRefusal;
use memless_engine::read;

#[test]
fn refuses_a_path_with_no_file() {
    let error = read("/no/such/memless/path.yaml").expect_err("refusal");
    assert!(matches!(error, SourceRefusal::PathHasNoFile { .. }));
}

#[test]
fn refuses_a_blank_file_as_empty() {
    let path = temp_file("blank.yaml", "   \n\t\n");
    let error = read(&read_path(&path)).expect_err("refusal");
    assert!(matches!(error, SourceRefusal::FileEmpty { .. }));
}

#[test]
fn refuses_an_unparsable_file_with_a_position() {
    let path = temp_file("broken.yaml", "a: [1, 2\n");
    let error = read(&read_path(&path)).expect_err("refusal");
    match error {
        SourceRefusal::InvalidYaml { at, .. } => assert!(at.is_some()),
        other => panic!("expected InvalidYaml, got {other:?}"),
    }
}

#[test]
fn refuses_several_documents_as_invalid_yaml() {
    let path = temp_file("multidoc.yaml", "a: 1\n---\nb: 2\n");
    let error = read(&read_path(&path)).expect_err("refusal");
    assert!(matches!(error, SourceRefusal::InvalidYaml { .. }));
}

#[test]
fn refuses_a_utf8_bom_as_invalid_yaml() {
    let path = temp_file("bom.yaml", "\u{FEFF}a: 1\n");
    let error = read(&read_path(&path)).expect_err("refusal");
    assert!(matches!(error, SourceRefusal::InvalidYaml { .. }));
}

#[test]
fn refuses_an_unreadable_file_as_not_readable() {
    use std::os::unix::fs::PermissionsExt;
    let path = temp_file("unreadable.yaml", "a: 1\n");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let result = read(&read_path(&path));
    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644));
    // Under a uid that bypasses permission bits (e.g. root in CI), the read
    // succeeds and there is nothing to assert; otherwise it must be A2.
    if let Err(error) = result {
        assert!(matches!(error, SourceRefusal::FileNotReadable { .. }));
    }
}

#[test]
fn builds_a_raw_document_with_its_provenance() {
    let path = temp_file("provenance.yaml", "users:\n  - id: 1\n");
    let document = read(&read_path(&path)).expect("document");
    assert_eq!(document.source, read_path(&path));
}

#[test]
fn accepts_bounded_anchors_and_aliases_without_a_numeric_refusal() {
    let path = temp_file("anchors.yaml", "users:\n  - &a {id: 1}\n  - *a\n");
    assert!(read(&read_path(&path)).is_ok());
}
