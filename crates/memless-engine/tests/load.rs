mod common;

use common::{one_row_without_id, one_user_document};
use memless_domain::document::RawDocument;
use memless_domain::refusal::SourceRefusal;
use memless_engine::{load, Refusal};

fn read_ok(_path: &str) -> Result<RawDocument, SourceRefusal> {
    Ok(one_user_document("stub"))
}

fn read_source_fail(_path: &str) -> Result<RawDocument, SourceRefusal> {
    Err(SourceRefusal::FileEmpty { path: "stub".to_string() })
}

fn read_incoherent(_path: &str) -> Result<RawDocument, SourceRefusal> {
    Ok(one_row_without_id("stub"))
}

#[test]
fn returns_the_base_when_read_yields_an_acceptable_document() {
    assert!(load(read_ok, "any").is_ok());
}

#[test]
fn returns_a_source_refusal_when_read_fails() {
    match load(read_source_fail, "any") {
        Err(Refusal::Source(_)) => {}
        _ => panic!("expected a source refusal"),
    }
}

#[test]
fn returns_a_structure_refusal_when_the_document_is_incoherent() {
    match load(read_incoherent, "any") {
        Err(Refusal::Structure(_)) => {}
        _ => panic!("expected a structure refusal"),
    }
}
