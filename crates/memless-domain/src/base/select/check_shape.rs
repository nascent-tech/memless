use super::distinct_headers::distinct_headers;
use super::output_headers::output_headers;
use super::plan::Plan;
use super::unordered_aggregate::unordered_aggregate;
use crate::query::Select;
use crate::refusal::QueryRefusal;

pub(crate) fn check_shape(plan: &Plan, query: &Select) -> Result<(), QueryRefusal> {
    distinct_headers(&output_headers(plan, &query.items))?;
    unordered_aggregate(query)
}
