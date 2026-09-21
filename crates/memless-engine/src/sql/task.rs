use sqlparser::ast::Expr;

pub(crate) enum Task<'e> {
    Visit(&'e Expr),
    CombineAnd,
    CombineOr,
}
