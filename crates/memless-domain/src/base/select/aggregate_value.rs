use super::candidate::Candidate;
use super::count_present::count_present;
use super::plan::Plan;
use super::sum_column::sum_column;
use crate::query::Aggregate;
use crate::refusal::QueryRefusal;
use crate::scalar::Scalar;

pub(crate) fn aggregate_value<'a>(
    plan: &Plan<'a>,
    aggregate: &Aggregate,
    candidates: &[Candidate<'a>],
) -> Result<Option<Scalar>, QueryRefusal> {
    match aggregate {
        Aggregate::CountStar => Ok(Some(Scalar::Integer(candidates.len() as i64))),
        Aggregate::Count(column) => Ok(Some(Scalar::Integer(count_present(plan, column, candidates)))),
        Aggregate::Sum(column) => sum_column(plan, column, candidates),
    }
}
