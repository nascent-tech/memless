use sqlparser::ast::SetExpr;

pub(crate) fn body_label(body: &SetExpr) -> &'static str {
    match body {
        SetExpr::Query(_) => "subquery",
        SetExpr::SetOperation { .. } => "UNION",
        SetExpr::Values(_) => "VALUES",
        _ => "this query",
    }
}
