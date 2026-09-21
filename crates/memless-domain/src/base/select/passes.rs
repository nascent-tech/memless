use super::candidate::Candidate;
use super::eval::Eval;
use super::frame::Frame;
use super::plan::Plan;
use super::step::step;
use crate::query::Filter;

pub(crate) fn passes<'a, 'f>(plan: &Plan<'a>, candidate: &Candidate<'a>, filter: &'f Filter) -> bool {
    let eval = Eval { plan, candidate };
    let mut work: Vec<Frame<'f>> = vec![Frame::Enter(filter)];
    let mut results: Vec<bool> = Vec::new();
    while let Some(frame) = work.pop() {
        step(&eval, frame, &mut work, &mut results);
    }
    results.pop().unwrap_or(false)
}
