use memless_engine::{execute, parse, replace_file};

use crate::instances::with_instance_mut;
use crate::outcome::Outcome;

pub(crate) fn run_execute(handle: u64, sql: &str) -> Outcome {
    match with_instance_mut(handle, |instance| execute(parse, replace_file, instance, sql)) {
        Some(Ok(affected)) => Outcome::accepted(affected),
        Some(Err(refusal)) => Outcome::refused(refusal.to_string()),
        None => Outcome::invalid("unknown handle"),
    }
}
