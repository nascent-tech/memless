use sqlparser::ast::Expr;

pub(crate) enum Argument<'a> {
    Star,
    Column(&'a Expr),
}
