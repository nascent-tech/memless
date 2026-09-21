#![allow(dead_code)]

use std::path::{Path, PathBuf};

use memless_domain::document::{RawDocument, RawKey, RawNode, RawScalar, RawStyle};

pub fn temp_file(name: &str, content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("memless-engine-tests");
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let path = dir.join(name);
    std::fs::write(&path, content).expect("write temp file");
    path
}

pub fn read_path(path: &Path) -> String {
    path.to_str().expect("utf-8 path").to_string()
}

pub fn plain(lexeme: &str) -> RawNode {
    RawNode::Scalar(RawScalar { lexeme: lexeme.to_string(), style: RawStyle::Plain })
}

pub fn one_user_document(source: &str) -> RawDocument {
    let row = RawNode::Mapping(vec![(RawKey::Text("id".to_string()), plain("1"))]);
    let table = RawNode::Sequence(vec![row]);
    let root = RawNode::Mapping(vec![(RawKey::Text("users".to_string()), table)]);
    RawDocument { source: source.to_string(), root }
}

pub fn one_row_without_id(source: &str) -> RawDocument {
    let row = RawNode::Mapping(vec![(RawKey::Text("name".to_string()), plain("ada"))]);
    let table = RawNode::Sequence(vec![row]);
    let root = RawNode::Mapping(vec![(RawKey::Text("users".to_string()), table)]);
    RawDocument { source: source.to_string(), root }
}
