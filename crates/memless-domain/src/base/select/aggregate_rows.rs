use super::aggregate_cells::aggregate_cells;
use super::aggregate_header::aggregate_header;
use super::candidate::Candidate;
use super::plan::Plan;
use crate::query::Aggregate;
use crate::refusal::QueryRefusal;
use crate::rows::Rows;

pub(crate) fn aggregate_rows<'a>(
    plan: &Plan<'a>,
    aggregates: &[Aggregate],
    candidates: &[Candidate<'a>],
) -> Result<Rows, QueryRefusal> {
    let columns = aggregates.iter().map(|item| aggregate_header(plan, item)).collect();
    let cells = aggregate_cells(plan, aggregates, candidates)?;
    Ok(Rows { columns, rows: vec![cells] })
}
