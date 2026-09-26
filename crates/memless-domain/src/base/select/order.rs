use super::candidate::Candidate;
use super::compare_keys::compare_keys;
use super::one_type::one_type;
use super::plan::Plan;
use crate::query::OrderKey;
use crate::refusal::QueryRefusal;

pub(crate) fn order<'a>(
    plan: &Plan<'a>,
    candidates: Vec<Candidate<'a>>,
    keys: &[OrderKey],
) -> Result<Vec<Candidate<'a>>, QueryRefusal> {
    keys.iter().try_for_each(|key| one_type(plan, &candidates, &key.column))?;
    let mut ordered = candidates;
    ordered.sort_by(|left, right| compare_keys(plan, left, right, keys));
    Ok(ordered)
}
