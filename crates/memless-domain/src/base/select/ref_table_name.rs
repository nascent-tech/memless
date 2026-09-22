use super::plan::Plan;

pub(crate) fn ref_table_name(plan: &Plan, joined: bool) -> String {
    match (&plan.join, joined) {
        (Some(join), true) => join.table.name.clone(),
        _ => plan.from.name.clone(),
    }
}
