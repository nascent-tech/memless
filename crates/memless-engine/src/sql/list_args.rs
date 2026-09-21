use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{FunctionArg, FunctionArgumentList};

pub(crate) fn list_args(list: &FunctionArgumentList) -> Result<&[FunctionArg], QueryRefusal> {
    if list.duplicate_treatment.is_some() {
        return Err(outside("DISTINCT in an aggregate"));
    }
    if !list.clauses.is_empty() {
        return Err(outside("this aggregate"));
    }
    Ok(&list.args)
}
