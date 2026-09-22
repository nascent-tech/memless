use sqlparser::ast::Expr;

pub(crate) fn unnest(expr: &Expr) -> &Expr {
    let mut current = expr;
    while let Expr::Nested(inner) = current {
        current = inner;
    }
    current
}
