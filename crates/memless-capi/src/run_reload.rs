use memless_engine::{read, reload};

use crate::instances::with_instance_mut;
use crate::outcome::Outcome;

pub(crate) fn run_reload(handle: u64) -> Outcome {
    match with_instance_mut(handle, |instance| reload(read, instance)) {
        Some(Ok(())) => Outcome::accepted(0),
        Some(Err(refusal)) => Outcome::refused(refusal.to_string()),
        None => Outcome::invalid("unknown handle"),
    }
}
