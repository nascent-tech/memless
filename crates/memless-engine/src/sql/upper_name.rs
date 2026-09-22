use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Function;

pub(crate) fn upper_name(function: &Function) -> Result<String, QueryRefusal> {
    if function.name.0.len() != 1 {
        return Err(outside("this function"));
    }
    Ok(function.name.0[0].value.to_uppercase())
}
