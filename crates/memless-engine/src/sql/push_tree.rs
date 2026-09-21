use super::task::Task;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn push_tree<'e>(
    tasks: &mut Vec<Task<'e>>,
    combine: Task<'e>,
    left: &'e Expr,
    right: &'e Expr,
) -> Result<(), QueryRefusal> {
    tasks.push(combine);
    tasks.push(Task::Visit(right));
    tasks.push(Task::Visit(left));
    Ok(())
}
