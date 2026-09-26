mod common;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use common::{read_path, temp_file};
use memless_domain::refusal::TransactionRefusal;
use memless_domain::Scalar;
use memless_engine::{execute, load, parse, query, read, reload, replace_file, Instance, Refusal};

static COUNTER: AtomicU64 = AtomicU64::new(0);

const SHOP: &str = "\
users:
  - id: 1
    role: ADMIN
wallets:
  - id: w1
    user_id: 1
    balance: 100
";

const EDITED: &str = "\
users:
  - id: 1
    role: ADMIN
wallets:
  - id: w1
    user_id: 1
    balance: 7
";

const INCOHERENT: &str = "\
users:
  - role: ADMIN
";

fn instance(prefix: &str) -> (Instance, PathBuf) {
    let name = format!("{prefix}-{}.yaml", COUNTER.fetch_add(1, Ordering::Relaxed));
    let path = temp_file(&name, SHOP);
    (load(read, &read_path(&path)).unwrap(), path)
}

fn balance(instance: &Instance) -> Option<Scalar> {
    let rows = query(parse, instance, "SELECT balance FROM wallets WHERE id = 'w1'").unwrap();
    rows.rows[0][0].clone()
}

fn residue_of(path: &Path) -> PathBuf {
    let name = path.file_name().unwrap().to_str().unwrap();
    path.with_file_name(format!(".{name}.memless-tmp"))
}

#[test]
fn a_reload_picks_up_an_external_edit_of_the_file() {
    let (mut inst, path) = instance("reload-edit");
    std::fs::write(&path, EDITED).unwrap();
    reload(read, &mut inst).unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(7)));
}

#[test]
fn a_reload_restores_the_original_bytes_written_back_on_disk() {
    let (mut inst, path) = instance("reload-restore");
    execute(parse, replace_file, &mut inst, "UPDATE wallets SET balance = 50 WHERE id = 'w1'").unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(50)));
    std::fs::write(&path, SHOP).unwrap();
    reload(read, &mut inst).unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(100)));
}

#[test]
fn a_reload_writes_nothing_and_keeps_the_path() {
    let (mut inst, path) = instance("reload-no-write");
    let before_path = inst.path.clone();
    reload(read, &mut inst).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), SHOP);
    assert_eq!(inst.path, before_path);
    assert!(inst.transaction.is_none());
}

#[test]
fn a_reload_leaves_a_neighbouring_residue_untouched() {
    let (mut inst, path) = instance("reload-residue");
    let residue = residue_of(&path);
    std::fs::write(&residue, "leftover").unwrap();
    reload(read, &mut inst).unwrap();
    assert_eq!(std::fs::read_to_string(&residue).unwrap(), "leftover");
    std::fs::remove_file(&residue).unwrap();
}

#[test]
fn a_reload_is_refused_while_a_transaction_is_open() {
    let (mut inst, path) = instance("reload-open");
    execute(parse, replace_file, &mut inst, "BEGIN").unwrap();
    execute(parse, replace_file, &mut inst, "UPDATE wallets SET balance = 50 WHERE id = 'w1'").unwrap();
    std::fs::write(&path, EDITED).unwrap();
    let refusal = reload(read, &mut inst).unwrap_err();
    assert_eq!(refusal, Refusal::Transaction(TransactionRefusal::OpenDuringReload));
    assert_eq!(refusal.to_string(), "cannot reload while a transaction is open");
    assert_eq!(balance(&inst), Some(Scalar::Integer(50)));
}

#[test]
fn the_transaction_stays_usable_after_a_refused_reload() {
    let (mut inst, path) = instance("reload-open-commit");
    execute(parse, replace_file, &mut inst, "BEGIN").unwrap();
    execute(parse, replace_file, &mut inst, "UPDATE wallets SET balance = 50 WHERE id = 'w1'").unwrap();
    assert!(reload(read, &mut inst).is_err());
    execute(parse, replace_file, &mut inst, "UPDATE wallets SET balance = 60 WHERE id = 'w1'").unwrap();
    execute(parse, replace_file, &mut inst, "COMMIT").unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(60)));
    assert!(std::fs::read_to_string(&path).unwrap().contains("balance: 60"));
}

#[test]
fn a_rolled_back_transaction_lets_the_next_reload_through() {
    let (mut inst, path) = instance("reload-after-rollback");
    execute(parse, replace_file, &mut inst, "BEGIN").unwrap();
    assert!(reload(read, &mut inst).is_err());
    execute(parse, replace_file, &mut inst, "ROLLBACK").unwrap();
    std::fs::write(&path, EDITED).unwrap();
    reload(read, &mut inst).unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(7)));
}

#[test]
fn an_incoherent_file_is_refused_like_load_and_keeps_the_old_state() {
    let (mut inst, path) = instance("reload-incoherent");
    std::fs::write(&path, INCOHERENT).unwrap();
    let refusal = reload(read, &mut inst).unwrap_err();
    let expected = load(read, &read_path(&path)).err().unwrap();
    assert_eq!(refusal, expected);
    assert_eq!(refusal.to_string(), "row 1 in \"users\" has no id");
    assert_eq!(balance(&inst), Some(Scalar::Integer(100)));
}

#[test]
fn an_invalid_yaml_file_is_refused_like_load_and_keeps_the_old_state() {
    let (mut inst, path) = instance("reload-invalid-yaml");
    std::fs::write(&path, "a: [1, 2\n").unwrap();
    let refusal = reload(read, &mut inst).unwrap_err();
    let expected = load(read, &read_path(&path)).err().unwrap();
    assert_eq!(refusal, expected);
    assert_eq!(balance(&inst), Some(Scalar::Integer(100)));
}

#[test]
fn a_deleted_file_is_refused_naming_the_path_and_keeps_the_old_state() {
    let (mut inst, path) = instance("reload-missing");
    std::fs::remove_file(&path).unwrap();
    let refusal = reload(read, &mut inst).unwrap_err();
    assert_eq!(refusal.to_string(), format!("no file at path {:?}", read_path(&path)));
    assert_eq!(balance(&inst), Some(Scalar::Integer(100)));
}

#[test]
fn the_old_state_stays_writable_after_a_failed_reload() {
    let (mut inst, path) = instance("reload-failed-write");
    std::fs::write(&path, INCOHERENT).unwrap();
    assert!(reload(read, &mut inst).is_err());
    execute(parse, replace_file, &mut inst, "UPDATE wallets SET balance = 30 WHERE id = 'w1'").unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(30)));
}

#[test]
fn a_reload_succeeds_after_a_failed_one() {
    let (mut inst, path) = instance("reload-retry");
    std::fs::remove_file(&path).unwrap();
    assert!(reload(read, &mut inst).is_err());
    std::fs::write(&path, EDITED).unwrap();
    reload(read, &mut inst).unwrap();
    assert_eq!(balance(&inst), Some(Scalar::Integer(7)));
}
