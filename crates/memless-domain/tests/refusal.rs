use std::error::Error;

use memless_domain::refusal::{RowLabel, SourceRefusal, StructureRefusal, TextPosition};
use memless_domain::{Id, Refusal, Scalar};

fn path() -> String {
    "x.yaml".to_string()
}

fn users() -> String {
    "users".to_string()
}

#[test]
fn renders_the_source_refusals() {
    assert_eq!(SourceRefusal::PathHasNoFile { path: path() }.to_string(), "no file at path \"x.yaml\"");
    let unreadable = SourceRefusal::FileNotReadable { path: path() };
    assert_eq!(unreadable.to_string(), "file at path \"x.yaml\" is not readable");
    assert_eq!(SourceRefusal::FileEmpty { path: path() }.to_string(), "file at path \"x.yaml\" is empty");
}

#[test]
fn renders_invalid_yaml_with_and_without_position() {
    let bare = SourceRefusal::InvalidYaml { path: path(), at: None };
    assert_eq!(bare.to_string(), "invalid YAML in \"x.yaml\"");
    let located = SourceRefusal::InvalidYaml { path: path(), at: Some(TextPosition { line: 3, column: 5 }) };
    assert_eq!(located.to_string(), "invalid YAML in \"x.yaml\" at line 3, column 5");
}

#[test]
fn renders_the_shape_refusals() {
    assert_eq!(StructureRefusal::NoTableDeclared { source: path() }.to_string(), "no table declared in \"x.yaml\"");
    assert_eq!(StructureRefusal::TableNotRowList { table: users() }.to_string(), "table \"users\" is not a list of rows");
    let row = StructureRefusal::RowNotFieldSet { table: users(), position: 2 };
    assert_eq!(row.to_string(), "row 2 in \"users\" is not a field set");
    assert_eq!(StructureRefusal::DuplicateTableKey { table: users() }.to_string(), "duplicate table key \"users\"");
}

#[test]
fn renders_the_row_and_column_shape_refusals() {
    let nested = StructureRefusal::NestedValue {
        table: users(),
        row: RowLabel::Position(1),
        column: "tags".to_string(),
    };
    assert_eq!(nested.to_string(), "nested value in \"tags\" of row 1 in \"users\"");
    let duplicate = StructureRefusal::DuplicateColumnKey {
        table: users(),
        row: RowLabel::Position(1),
        column: "name".to_string(),
    };
    assert_eq!(duplicate.to_string(), "\"name\" duplicated in row 1 of \"users\"");
}

#[test]
fn renders_the_non_text_key_refusal_with_and_without_a_table() {
    let top = StructureRefusal::NonTextKey { rendered: "5".to_string(), table: None, position: 1 };
    assert_eq!(top.to_string(), "non-text key 5 at position 1");
    let column = StructureRefusal::NonTextKey { rendered: "5".to_string(), table: Some(users()), position: 2 };
    assert_eq!(column.to_string(), "non-text key 5 at position 2 in table \"users\"");
}

#[test]
fn renders_the_coherence_refusals() {
    assert_eq!(StructureRefusal::MissingId { table: users(), position: 1 }.to_string(), "row 1 in \"users\" has no id");
    let bad = StructureRefusal::IdNotTextOrInteger { table: users(), position: 1, value: Scalar::Decimal(1.25) };
    assert_eq!(bad.to_string(), "row 1 in \"users\" has a non-text non-integer id: 1.25");
    let duplicate = StructureRefusal::DuplicateId { table: users(), id: Id::Integer(5), positions: (1, 2) };
    assert_eq!(duplicate.to_string(), "duplicate id 5 in \"users\" at rows 1 and 2");
}

#[test]
fn renders_the_broken_relation_refusal() {
    let broken = StructureRefusal::BrokenRelation {
        table: "orders".to_string(),
        row: RowLabel::Id(Id::Integer(1)),
        column: "user_id".to_string(),
        target_table: users(),
        value: Scalar::Integer(9),
    };
    assert_eq!(broken.to_string(), "broken relation \"user_id\" of row 1 in \"orders\": no row 9 in \"users\"");
}

#[test]
fn converts_layered_refusals_into_refusal_and_exposes_error() {
    let source: Refusal = SourceRefusal::FileEmpty { path: path() }.into();
    assert_eq!(source.to_string(), "file at path \"x.yaml\" is empty");
    let structure: Refusal = StructureRefusal::DuplicateTableKey { table: users() }.into();
    assert_eq!(structure.to_string(), "duplicate table key \"users\"");
    let as_error: &dyn Error = &structure;
    assert_eq!(as_error.to_string(), "duplicate table key \"users\"");
}
