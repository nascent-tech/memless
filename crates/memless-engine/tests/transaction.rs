mod common;

use std::sync::atomic::{AtomicU64, Ordering};

use common::{read_path, temp_file};
use memless_domain::refusal::WriteRefusal;
use memless_domain::Scalar;
use memless_engine::{execute, load, parse, query, read, render, replace_file, Instance, Refusal};

static COUNTER: AtomicU64 = AtomicU64::new(0);

const SHOP: &str = "\
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

const AFTER_TRANSFER: &str = "\
users:
  - id: 1
    role: ADMIN
  - id: 2
    role: USER
wallets:
  - id: w1
    user_id: 1
    balance: 50
  - id: w2
    user_id: 2
    balance: 100
";

fn boom(_path: &str, _text: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "boom"))
}

fn instance(prefix: &str) -> Instance {
    let name = format!("{prefix}-{}.yaml", COUNTER.fetch_add(1, Ordering::Relaxed));
    let path = temp_file(&name, SHOP);
    load(read, &read_path(&path)).unwrap()
}

fn balance(instance: &Instance, id: &str) -> Option<Scalar> {
    let rows = query(parse, instance, &format!("SELECT balance FROM wallets WHERE id = '{id}'")).unwrap();
    rows.rows[0][0].clone()
}

#[test]
fn a_transfer_commits_once_and_reads_its_own_writes() {
    let mut inst = instance("txn-transfer");
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    assert_eq!(execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 50 WHERE id = 'w1'").unwrap(), 1);
    assert_eq!(execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 100 WHERE id = 'w2'").unwrap(), 1);
    assert_eq!(balance(&inst, "w1"), Some(Scalar::Integer(50)));
    execute(parse, replace_file, &mut inst, "COMMIT").unwrap();
    let on_disk = std::fs::read_to_string(&inst.path).unwrap();
    assert_eq!(on_disk, AFTER_TRANSFER);
    assert_eq!(render(&inst.base.document()), on_disk);
    assert_eq!(balance(&inst, "w2"), Some(Scalar::Integer(100)));
}

#[test]
fn a_commit_without_a_change_touches_no_disk() {
    let mut inst = instance("txn-commit-noop");
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 100 WHERE id = 'w1'").unwrap();
    assert_eq!(execute(parse, boom, &mut inst, "COMMIT").unwrap(), 0);
}

#[test]
fn intermediate_writes_never_touch_the_disk() {
    let mut inst = instance("txn-nodisk");
    let before = std::fs::read_to_string(&inst.path).unwrap();
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 1 WHERE id = 'w1'").unwrap();
    assert_eq!(std::fs::read_to_string(&inst.path).unwrap(), before);
}

#[test]
fn a_rollback_leaves_the_file_and_restores_the_state() {
    let mut inst = instance("txn-rollback");
    let before = std::fs::read_to_string(&inst.path).unwrap();
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 999 WHERE id = 'w1'").unwrap();
    execute(parse, boom, &mut inst, "ROLLBACK").unwrap();
    assert_eq!(std::fs::read_to_string(&inst.path).unwrap(), before);
    assert_eq!(balance(&inst, "w1"), Some(Scalar::Integer(100)));
}

#[test]
fn a_refused_instruction_leaves_the_transaction_open() {
    let mut inst = instance("txn-open");
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    let refusal = execute(parse, boom, &mut inst, "UPDATE ghosts SET x = 1").unwrap_err();
    assert!(matches!(refusal, Refusal::Query(_)));
    assert_eq!(execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 7 WHERE id = 'w1'").unwrap(), 1);
    assert_eq!(balance(&inst, "w1"), Some(Scalar::Integer(7)));
}

#[test]
fn an_invalid_intermediate_state_is_tolerated_until_commit() {
    let mut inst = instance("txn-tolerate");
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "DELETE FROM users WHERE id = 2").unwrap();
    execute(parse, boom, &mut inst, "DELETE FROM wallets WHERE id = 'w2'").unwrap();
    execute(parse, replace_file, &mut inst, "COMMIT").unwrap();
    assert_eq!(render(&inst.base.document()), std::fs::read_to_string(&inst.path).unwrap());
}

#[test]
fn a_failed_validation_closes_the_transaction_and_keeps_the_state() {
    let mut inst = instance("txn-invalid");
    let before = std::fs::read_to_string(&inst.path).unwrap();
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "DELETE FROM users WHERE id = 2").unwrap();
    let refusal = execute(parse, replace_file, &mut inst, "COMMIT").unwrap_err();
    assert!(matches!(refusal, Refusal::Structure(_)));
    assert_eq!(std::fs::read_to_string(&inst.path).unwrap(), before);
    let rows = query(parse, &inst, "SELECT role FROM users WHERE id = 2").unwrap();
    assert_eq!(rows.rows.len(), 1);
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
}

#[test]
fn a_disk_failure_at_commit_closes_the_transaction_and_keeps_memory() {
    let mut inst = instance("txn-disk");
    let before = std::fs::read_to_string(&inst.path).unwrap();
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 5 WHERE id = 'w1'").unwrap();
    let refusal = execute(parse, boom, &mut inst, "COMMIT").unwrap_err();
    assert!(matches!(refusal, Refusal::Write(WriteRefusal::DiskWriteFailed { .. })));
    assert_eq!(std::fs::read_to_string(&inst.path).unwrap(), before);
    assert_eq!(balance(&inst, "w1"), Some(Scalar::Integer(100)));
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
}

#[test]
fn a_second_begin_is_refused_with_the_transaction_left_open() {
    let mut inst = instance("txn-twice");
    execute(parse, boom, &mut inst, "BEGIN").unwrap();
    let refusal = execute(parse, boom, &mut inst, "BEGIN").unwrap_err();
    assert_eq!(refusal.to_string(), "a transaction is already open");
    assert_eq!(execute(parse, boom, &mut inst, "UPDATE wallets SET balance = 3 WHERE id = 'w1'").unwrap(), 1);
}

#[test]
fn a_commit_or_rollback_without_a_transaction_is_refused() {
    let mut inst = instance("txn-none");
    assert_eq!(execute(parse, replace_file, &mut inst, "COMMIT").unwrap_err().to_string(), "no open transaction");
    assert_eq!(execute(parse, boom, &mut inst, "ROLLBACK").unwrap_err().to_string(), "no open transaction");
}
