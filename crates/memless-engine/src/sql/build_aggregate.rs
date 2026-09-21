use super::argument::Argument;
use super::column_ref_of::column_ref_of;
use super::outside::outside;
use memless_domain::query::Aggregate;
use memless_domain::QueryRefusal;

pub(crate) fn build_aggregate(name: &str, argument: Argument, has_join: bool) -> Result<Aggregate, QueryRefusal> {
    match (name, argument) {
        ("COUNT", Argument::Star) => Ok(Aggregate::CountStar),
        ("COUNT", Argument::Column(expr)) => Ok(Aggregate::Count(column_ref_of(expr, has_join)?)),
        ("SUM", Argument::Column(expr)) => Ok(Aggregate::Sum(column_ref_of(expr, has_join)?)),
        _ => Err(outside("this aggregate")),
    }
}
