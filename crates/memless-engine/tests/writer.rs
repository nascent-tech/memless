use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use memless_engine::replace_file;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn workdir(prefix: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("memless-writer-tests")
        .join(format!("{prefix}-{}", COUNTER.fetch_add(1, Ordering::Relaxed)));
    fs::create_dir_all(&dir).expect("create work dir");
    dir
}

fn residue(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!(".{name}.memless-tmp"))
}

#[test]
fn writes_the_text_and_reads_it_back() {
    let dir = workdir("write");
    let path = dir.join("data.yaml");
    replace_file(path.to_str().unwrap(), "users: []\n").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "users: []\n");
    assert!(!residue(&dir, "data.yaml").exists());
}

#[test]
fn overwrites_a_stale_residue() {
    let dir = workdir("stale");
    let path = dir.join("data.yaml");
    fs::write(residue(&dir, "data.yaml"), "leftover").expect("seed residue");
    replace_file(path.to_str().unwrap(), "fresh\n").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "fresh\n");
    assert!(!residue(&dir, "data.yaml").exists());
}

#[test]
fn replaces_an_existing_file_in_place() {
    let dir = workdir("replace");
    let path = dir.join("data.yaml");
    fs::write(&path, "before\n").expect("seed file");
    replace_file(path.to_str().unwrap(), "after\n").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "after\n");
}

#[cfg(unix)]
#[test]
fn a_failure_leaves_the_original_intact_and_no_residue() {
    use std::os::unix::fs::PermissionsExt;
    let dir = workdir("locked");
    let path = dir.join("data.yaml");
    fs::write(&path, "original\n").expect("seed file");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o555)).expect("lock dir");
    let outcome = replace_file(path.to_str().unwrap(), "new\n");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).expect("unlock dir");
    assert!(outcome.is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "original\n");
    assert!(!residue(&dir, "data.yaml").exists());
}

#[cfg(unix)]
#[test]
fn keeps_the_permissions_of_the_original() {
    use std::os::unix::fs::PermissionsExt;
    let dir = workdir("perms");
    let path = dir.join("data.yaml");
    fs::write(&path, "before\n").expect("seed file");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).expect("set mode");
    replace_file(path.to_str().unwrap(), "after\n").unwrap();
    let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o640);
}
