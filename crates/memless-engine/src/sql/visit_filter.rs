use super::boolean_parts::boolean_parts;
use super::push_leaf::push_leaf;
use super::push_tree::push_tree;
use super::task::Task;
use super::unnest::unnest;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn visit_filter<'e>(
    expr: &'e Expr,
    has_join: bool,
    tasks: &mut Vec<Task<'e>>,
    output: &mut Vec<Filter>,
) -> Result<(), QueryRefusal> {
    let expr = unnest(expr);
    match boolean_parts(expr) {
        Some((combine, left, right)) => push_tree(tasks, combine, left, right),
        None => push_leaf(output, expr, has_join),
    }
}
