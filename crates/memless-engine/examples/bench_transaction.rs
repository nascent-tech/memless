use std::time::{Duration, Instant};

use memless_engine::{execute, load, parse, read, replace_file, Instance};

const WRITES: [usize; 4] = [1, 10, 100, 1000];
const REPS: usize = 20;
const P50: usize = 50;
const P95: usize = 95;

fn main() {
    println!("k\ttxn_us_p50\ttxn_us_p95\tisolated_us_p50\tisolated_us_p95");
    WRITES.iter().for_each(|writes| println!("{}", report(*writes)));
}

fn report(writes: usize) -> String {
    let (txn50, txn95) = sample(writes, time_transaction);
    let (iso50, iso95) = sample(writes, time_isolated);
    format!("{writes}\t{txn50}\t{txn95}\t{iso50}\t{iso95}")
}

fn sample(writes: usize, measure: fn(usize, &str) -> Duration) -> (u128, u128) {
    let text = seed_text();
    let durations: Vec<u128> = (0..REPS).map(|_| measure(writes, &text).as_micros()).collect();
    (percentile(&durations, P50), percentile(&durations, P95))
}

fn seed_text() -> String {
    "rows:\n  - id: 1\n    val: v1\n".to_string()
}

fn time_transaction(writes: usize, text: &str) -> Duration {
    let (dir, mut instance) = fresh(text);
    let start = Instant::now();
    run_transaction(&mut instance, writes);
    let elapsed = start.elapsed();
    drop(instance);
    let _ = std::fs::remove_dir_all(&dir);
    elapsed
}

fn run_transaction(instance: &mut Instance, writes: usize) {
    execute(parse, replace_file, instance, "BEGIN").expect("begin");
    (0..writes).for_each(|turn| apply(instance, turn));
    execute(parse, replace_file, instance, "COMMIT").expect("commit");
}

fn time_isolated(writes: usize, text: &str) -> Duration {
    let (dir, mut instance) = fresh(text);
    let start = Instant::now();
    (0..writes).for_each(|turn| apply(&mut instance, turn));
    let elapsed = start.elapsed();
    drop(instance);
    let _ = std::fs::remove_dir_all(&dir);
    elapsed
}

fn apply(instance: &mut Instance, turn: usize) {
    let sql = format!("UPDATE rows SET val = 'b{turn}' WHERE id = 1");
    execute(parse, replace_file, instance, &sql).expect("write");
}

fn fresh(text: &str) -> (std::path::PathBuf, Instance) {
    let (path, dir) = seed(text);
    let instance = load(read, &path).expect("load");
    drop(path);
    (dir, instance)
}

fn seed(text: &str) -> (String, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("memless-bench-txn-{}", unique()));
    std::fs::create_dir_all(&dir).expect("create dir");
    let path = dir.join("bench.yaml");
    std::fs::write(&path, text).expect("seed");
    (path.to_string_lossy().into_owned(), dir)
}

fn unique() -> u128 {
    Instant::now().elapsed().as_nanos() ^ (std::process::id() as u128).rotate_left(17) ^ rolling()
}

fn rolling() -> u128 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed) as u128
}

fn percentile(durations: &[u128], rank: usize) -> u128 {
    let mut sorted = durations.to_vec();
    sorted.sort_unstable();
    let index = (rank * (sorted.len() - 1)) / 100;
    sorted[index]
}
