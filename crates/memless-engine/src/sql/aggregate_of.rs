use super::build_aggregate::build_aggregate;
use super::reject_function_modifiers::reject_function_modifiers;
use super::single_argument::single_argument;
use super::upper_name::upper_name;
use memless_domain::query::Aggregate;
use memless_domain::QueryRefusal;
use sqlparser::ast::Function;

pub(crate) fn aggregate_of(function: &Function, has_join: bool) -> Result<Aggregate, QueryRefusal> {
    reject_function_modifiers(function)?;
    let name = upper_name(function)?;
    let argument = single_argument(function)?;
    build_aggregate(&name, argument, has_join)
}
