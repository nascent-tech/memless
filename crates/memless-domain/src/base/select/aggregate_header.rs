use super::header_name::header_name;
use super::plan::Plan;
use crate::query::Aggregate;

pub(crate) fn aggregate_header(plan: &Plan, aggregate: &Aggregate) -> String {
    match aggregate {
        Aggregate::CountStar => "COUNT(*)".to_string(),
        Aggregate::Count(column) => format!("COUNT({})", header_name(plan, column)),
        Aggregate::Sum(column) => format!("SUM({})", header_name(plan, column)),
    }
}
