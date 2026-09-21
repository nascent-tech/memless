use super::argument::Argument;
use super::argument_list::argument_list;
use super::argument_of::argument_of;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Function;

pub(crate) fn single_argument<'a>(function: &'a Function) -> Result<Argument<'a>, QueryRefusal> {
    let args = argument_list(function)?;
    if args.len() != 1 {
        return Err(outside("this aggregate"));
    }
    argument_of(&args[0])
}
