use super::candidate::Candidate;
use super::eval::Eval;
use super::plan::Plan;
use crate::base::filter::keeps;
use crate::query::Select;

pub(crate) fn keep<'a>(plan: &Plan<'a>, candidates: Vec<Candidate<'a>>, query: &Select) -> Vec<Candidate<'a>> {
    match &query.filter {
        None => candidates,
        Some(filter) => candidates.into_iter().filter(|candidate| keeps(&Eval { plan, candidate }, filter)).collect(),
    }
}
