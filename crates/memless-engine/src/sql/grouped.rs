use sqlparser::ast::GroupByExpr;

pub(crate) fn grouped(group_by: &GroupByExpr) -> bool {
    match group_by {
        GroupByExpr::Expressions(exprs, _) => !exprs.is_empty(),
        _ => true,
    }
}
