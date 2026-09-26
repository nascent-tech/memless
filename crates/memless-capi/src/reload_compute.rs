use crate::ffi::guard;
use crate::outcome::Outcome;
use crate::run_reload::run_reload;

pub(crate) fn reload_compute(handle: u64) -> Outcome {
    guard(Outcome::internal(), || run_reload(handle))
}
