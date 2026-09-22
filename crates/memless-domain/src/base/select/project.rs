use super::aggregate_rows::aggregate_rows;
use super::candidate::Candidate;
use super::column_rows::column_rows;
use super::plan::Plan;
use super::projected_cols::projected_cols;
use crate::query::{Items, Select};
use crate::refusal::QueryRefusal;
use crate::rows::Rows;

pub(crate) fn project<'a>(plan: &Plan<'a>, query: &Select, candidates: &[Candidate<'a>]) -> Result<Rows, QueryRefusal> {
    if let Items::Aggregates(aggregates) = &query.items {
        return aggregate_rows(plan, aggregates, candidates);
    }
    let cols = projected_cols(plan, &query.items);
    Ok(column_rows(&cols, candidates))
}
