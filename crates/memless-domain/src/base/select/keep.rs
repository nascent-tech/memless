use super::candidate::Candidate;
use super::passes::passes;
use super::plan::Plan;
use crate::query::Select;

pub(crate) fn keep<'a>(plan: &Plan<'a>, candidates: Vec<Candidate<'a>>, query: &Select) -> Vec<Candidate<'a>> {
    match &query.filter {
        None => candidates,
        Some(filter) => candidates.into_iter().filter(|candidate| passes(plan, candidate, filter)).collect(),
    }
}
