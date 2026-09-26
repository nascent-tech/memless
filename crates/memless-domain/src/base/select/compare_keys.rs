use std::cmp::Ordering;

use super::candidate::Candidate;
use super::compare_key::compare_key;
use super::plan::Plan;
use crate::query::OrderKey;

pub(crate) fn compare_keys(plan: &Plan, left: &Candidate, right: &Candidate, keys: &[OrderKey]) -> Ordering {
    keys.iter()
        .map(|key| compare_key(plan, left, right, key))
        .find(|ordering| ordering.is_ne())
        .unwrap_or(Ordering::Equal)
}
