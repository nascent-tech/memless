use super::aggregate_value::aggregate_value;
use super::candidate::Candidate;
use super::plan::Plan;
use crate::query::Aggregate;
use crate::refusal::QueryRefusal;
use crate::scalar::Scalar;

pub(crate) fn aggregate_cells<'a>(
    plan: &Plan<'a>,
    aggregates: &[Aggregate],
    candidates: &[Candidate<'a>],
) -> Result<Vec<Option<Scalar>>, QueryRefusal> {
    aggregates.iter().map(|item| aggregate_value(plan, item, candidates)).collect()
}
