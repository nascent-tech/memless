mod common;

use std::sync::atomic::{AtomicU64, Ordering};

use common::{read_path, temp_file};
use memless_domain::refusal::{QueryRefusal, WriteRefusal};
use memless_engine::{execute, load, parse, read, replace_file, render, Instance, Refusal};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn boom(_path: &str, _text: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "boom"))
}

fn instance(prefix: &str, yaml: &str) -> Instance {
    let name = format!("{prefix}-{}.yaml", COUNTER.fetch_add(1, Ordering::Relaxed));
    let path = temp_file(&name, yaml);
    load(read, &read_path(&path)).unwrap()
}

#[test]
fn an_accepted_write_rewrites_the_file_byte_for_byte() {
    let mut inst = instance("write-insert", "logs:\n  - id: 1\n    line: a\n");
    let affected = execute(parse, replace_file, &mut inst, "INSERT INTO logs (id, line) VALUES (2, 'b')").unwrap();
    assert_eq!(affected, 1);
    assert_eq!(std::fs::read_to_string(&inst.path).unwrap(), "logs:\n  - id: 1\n    line: a\n  - id: 2\n    line: b\n");
    assert_eq!(render(&inst.base.document()), std::fs::read_to_string(&inst.path).unwrap());
}

#[test]
fn a_write_without_a_change_touches_no_disk() {
    let mut inst = instance("write-noop", "logs:\n  - id: 1\n    line: a\n");
    let affected = execute(parse, boom, &mut inst, "UPDATE logs SET line = 'a' WHERE id = 1").unwrap();
    assert_eq!(affected, 1);
}

#[test]
fn a_disk_failure_leaves_memory_behind() {
    let mut inst = instance("write-fail", "logs:\n  - id: 1\n    line: a\n");
    let before = inst.base.clone();
    let refusal = execute(parse, boom, &mut inst, "UPDATE logs SET line = 'z' WHERE id = 1").unwrap_err();
    assert!(matches!(refusal, Refusal::Write(WriteRefusal::DiskWriteFailed { .. })));
    assert!(inst.base == before);
}

#[test]
fn a_select_passed_to_execute_is_refused() {
    let mut inst = instance("write-select", "logs:\n  - id: 1\n    line: a\n");
    let refusal = execute(parse, replace_file, &mut inst, "SELECT * FROM logs").unwrap_err();
    assert!(matches!(refusal, Refusal::Query(QueryRefusal::OutsideSubset { .. })));
}
