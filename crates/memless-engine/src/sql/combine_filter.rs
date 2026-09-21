use super::joined_filter::joined_filter;
use super::outside::outside;
use memless_domain::query::Filter;
use memless_domain::QueryRefusal;

pub(crate) fn combine_filter(output: &mut Vec<Filter>, is_and: bool) -> Result<(), QueryRefusal> {
    let right = output.pop().ok_or_else(|| outside("empty filter"))?;
    let left = output.pop().ok_or_else(|| outside("empty filter"))?;
    output.push(joined_filter(is_and, left, right));
    Ok(())
}
