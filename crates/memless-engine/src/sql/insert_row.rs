use super::outside::outside;
use super::value_of::value_of;
use super::values_body::values_row;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::Query;

pub(crate) fn insert_row(source: Option<Box<Query>>) -> Result<Vec<Option<Scalar>>, QueryRefusal> {
    let query = source.ok_or_else(|| outside("INSERT without VALUES"))?;
    let row = values_row(*query)?;
    row.iter().map(value_of).collect()
}
