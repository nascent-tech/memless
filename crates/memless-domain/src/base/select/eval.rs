use super::candidate::Candidate;
use super::plan::Plan;

pub(crate) struct Eval<'e, 'a> {
    pub plan: &'e Plan<'a>,
    pub candidate: &'e Candidate<'a>,
}
