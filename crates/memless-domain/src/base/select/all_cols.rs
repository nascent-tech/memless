use super::col::Col;
use super::plan::Plan;
use super::table_cols::table_cols;

pub(crate) fn all_cols(plan: &Plan) -> Vec<Col> {
    let has_join = plan.join.is_some();
    let mut cols = table_cols(plan.from, false, has_join);
    if let Some(join) = &plan.join {
        cols.extend(table_cols(join.table, true, true));
    }
    cols
}
