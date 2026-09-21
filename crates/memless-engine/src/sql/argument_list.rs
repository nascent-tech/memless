use super::list_args::list_args;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{FunctionArg, FunctionArguments};

pub(crate) fn argument_list(function: &sqlparser::ast::Function) -> Result<&[FunctionArg], QueryRefusal> {
    match &function.args {
        FunctionArguments::List(list) => list_args(list),
        _ => Err(outside("this aggregate")),
    }
}
