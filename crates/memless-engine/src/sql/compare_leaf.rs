use super::binary_op::binary_op;
use super::column_ref_of::column_ref_of;
use super::literal_of::literal_of;
use memless_domain::query::{Compare, Filter};
use memless_domain::QueryRefusal;
use sqlparser::ast::{BinaryOperator, Expr};

pub(crate) fn compare_leaf(
    left: &Expr,
    op: &BinaryOperator,
    right: &Expr,
    has_join: bool,
) -> Result<Filter, QueryRefusal> {
    let column = column_ref_of(left, has_join)?;
    let comparison = binary_op(op)?;
    let literal = literal_of(right)?;
    Ok(Filter::Compare(Compare { column, op: comparison, literal }))
}
