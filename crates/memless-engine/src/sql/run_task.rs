use super::combine_filter::combine_filter;
use super::task::Task;
use super::visit_filter::visit_filter;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;

pub(crate) fn run_task<'e>(
    task: Task<'e>,
    has_join: bool,
    tasks: &mut Vec<Task<'e>>,
    output: &mut Vec<Filter>,
) -> Result<(), QueryRefusal> {
    match task {
        Task::Visit(expr) => visit_filter(expr, has_join, tasks, output),
        Task::CombineAnd => combine_filter(output, true),
        Task::CombineOr => combine_filter(output, false),
    }
}
