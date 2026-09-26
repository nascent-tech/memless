use std::cmp::Ordering;

use super::candidate::Candidate;
use super::cell::cell;
use super::directed::directed;
use super::plan::Plan;
use crate::query::OrderKey;
use crate::scalar::scalar_order::compare_same_type;

pub(crate) fn compare_key(plan: &Plan, left: &Candidate, right: &Candidate, key: &OrderKey) -> Ordering {
    match (cell(plan, left, &key.column), cell(plan, right, &key.column)) {
        (Some(first), Some(second)) => directed(key.direction, compare_same_type(first, second)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}
