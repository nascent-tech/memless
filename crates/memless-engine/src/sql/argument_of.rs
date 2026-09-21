use super::argument::Argument;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::{FunctionArg, FunctionArgExpr};

pub(crate) fn argument_of<'a>(arg: &'a FunctionArg) -> Result<Argument<'a>, QueryRefusal> {
    match arg {
        FunctionArg::Unnamed(FunctionArgExpr::Wildcard) => Ok(Argument::Star),
        FunctionArg::Unnamed(FunctionArgExpr::Expr(expr)) => Ok(Argument::Column(expr)),
        _ => Err(outside("this aggregate argument")),
    }
}
