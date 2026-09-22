use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{Function, FunctionArguments};

pub(crate) fn reject_function_modifiers(function: &Function) -> Result<(), QueryRefusal> {
    if function.over.is_some() {
        return Err(outside("window function"));
    }
    if function.filter.is_some() || function.null_treatment.is_some() {
        return Err(outside("this aggregate"));
    }
    if !function.within_group.is_empty() || !matches!(function.parameters, FunctionArguments::None) {
        return Err(outside("this aggregate"));
    }
    Ok(())
}
