use super::candidate::Candidate;
use super::joined_candidates::joined_candidates;
use super::plan::Plan;
use super::single_candidate::single_candidate;

pub(crate) fn combine<'a>(plan: &Plan<'a>) -> Vec<Candidate<'a>> {
    match &plan.join {
        None => plan.from.rows.iter().map(single_candidate).collect(),
        Some(join) => joined_candidates(plan.from, join),
    }
}
