use super::outside::outside;
use super::run_task::run_task;
use super::task::Task;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;
use sqlparser::ast::Expr;

pub(crate) fn lower_filter(expr: &Expr, has_join: bool) -> Result<Filter, QueryRefusal> {
    let mut tasks: Vec<Task> = vec![Task::Visit(expr)];
    let mut output: Vec<Filter> = Vec::new();
    while let Some(task) = tasks.pop() {
        run_task(task, has_join, &mut tasks, &mut output)?;
    }
    output.pop().ok_or_else(|| outside("empty filter"))
}
