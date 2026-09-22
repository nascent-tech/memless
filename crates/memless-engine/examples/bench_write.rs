use std::time::{Duration, Instant};

use memless_engine::{load, parse, read, replace_file, write, Instance};

const SIZES: [usize; 4] = [4, 100, 1000, 10000];
const REPS: usize = 20;
const SUITE: usize = 100;
const P50: usize = 50;
const P95: usize = 95;

fn main() {
    println!("k\tload_us_p50\tload_us_p95\twrite_us_p50\twrite_us_p95\tsuite100_us_p50\tsuite100_us_p95");
    SIZES.iter().for_each(|size| println!("{}", report(*size)));
}

fn report(size: usize) -> String {
    let (load50, load95) = sample(size, time_load);
    let (write50, write95) = sample(size, time_write);
    let (suite50, suite95) = sample(size, time_suite);
    format!("{size}\t{load50}\t{load95}\t{write50}\t{write95}\t{suite50}\t{suite95}")
}

fn sample(size: usize, measure: fn(usize, &str) -> Duration) -> (u128, u128) {
    let text = generate(size);
    let durations: Vec<u128> = (0..REPS).map(|_| measure(size, &text).as_micros()).collect();
    (percentile(&durations, P50), percentile(&durations, P95))
}

fn generate(size: usize) -> String {
    let body: String = (1..=size).map(|index| format!("  - id: {index}\n    val: v{index}\n")).collect();
    format!("rows:\n{body}")
}

fn time_load(_size: usize, text: &str) -> Duration {
    let (path, dir) = seed(text);
    let start = Instant::now();
    let instance = load(read, &path).expect("load");
    let elapsed = start.elapsed();
    drop(instance);
    let _ = std::fs::remove_dir_all(&dir);
    elapsed
}

fn time_write(_size: usize, text: &str) -> Duration {
    let (dir, mut instance) = fresh(text);
    let start = Instant::now();
    write(parse, replace_file, &mut instance, "UPDATE rows SET val = 'bench' WHERE id = 1").expect("write");
    let elapsed = start.elapsed();
    drop(instance);
    let _ = std::fs::remove_dir_all(&dir);
    elapsed
}

fn time_suite(_size: usize, text: &str) -> Duration {
    let (dir, mut instance) = fresh(text);
    let start = Instant::now();
    (0..SUITE).for_each(|turn| apply(&mut instance, turn));
    let elapsed = start.elapsed();
    drop(instance);
    let _ = std::fs::remove_dir_all(&dir);
    elapsed
}

fn apply(instance: &mut Instance, turn: usize) {
    let sql = format!("UPDATE rows SET val = 'b{turn}' WHERE id = 1");
    write(parse, replace_file, instance, &sql).expect("suite write");
}

fn fresh(text: &str) -> (std::path::PathBuf, Instance) {
    let (path, dir) = seed(text);
    let instance = load(read, &path).expect("load");
    drop(path);
    (dir, instance)
}

fn seed(text: &str) -> (String, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("memless-bench-{}", unique()));
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
