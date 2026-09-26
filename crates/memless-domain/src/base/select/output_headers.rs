use super::aggregate_header::aggregate_header;
use super::plan::Plan;
use super::projected_cols::projected_cols;
use crate::query::Items;

pub(crate) fn output_headers(plan: &Plan, items: &Items) -> Vec<String> {
    match items {
        Items::Aggregates(aggregates) => aggregates.iter().map(|item| aggregate_header(plan, item)).collect(),
        _ => projected_cols(plan, items).into_iter().map(|col| col.header).collect(),
    }
}
