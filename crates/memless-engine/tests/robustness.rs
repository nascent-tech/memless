mod common;

use common::{read_path, temp_file};
use memless_engine::read;

// A recursive Drop or a recursive parser would overflow the test thread's
// stack (2 MiB) and abort the process on this input; the iterative builder
// and the iterative Drop of RawNode must both survive it.
#[test]
fn parses_and_drops_a_deeply_nested_document_without_overflowing() {
    let depth = 200_000;
    let nested = format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    let path = temp_file("deep.yaml", &format!("root: {nested}\n"));
    let document = read(&read_path(&path));
    assert!(document.is_ok());
    drop(document);
}
