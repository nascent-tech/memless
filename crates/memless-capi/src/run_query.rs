use memless_engine::{parse, query};

use crate::instances::with_base;
use crate::outcome::Outcome;
use crate::prepare::prepare;
use crate::results::store_result;

pub(crate) fn run_query(handle: u64, sql: &str) -> Outcome {
    match with_base(handle, |base| query(parse, base, sql)) {
        Some(Ok(rows)) => Outcome::accepted(store_result(prepare(rows))),
        Some(Err(refusal)) => Outcome::refused(refusal.to_string()),
        None => Outcome::invalid("unknown handle"),
    }
}
