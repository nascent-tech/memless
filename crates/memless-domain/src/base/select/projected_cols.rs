use super::all_cols::all_cols;
use super::col::Col;
use super::named_cols::named_cols;
use super::plan::Plan;
use crate::query::Items;

pub(crate) fn projected_cols(plan: &Plan, items: &Items) -> Vec<Col> {
    match items {
        Items::Columns(columns) => named_cols(plan, columns),
        _ => all_cols(plan),
    }
}
