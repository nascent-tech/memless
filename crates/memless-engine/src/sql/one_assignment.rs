use super::assignment_target::target_column;
use super::value_of::value_of;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::Assignment;

pub(crate) fn one_assignment(assignment: &Assignment) -> Result<(String, Option<Scalar>), QueryRefusal> {
    let column = target_column(&assignment.target)?;
    let value = value_of(&assignment.value)?;
    Ok((column, value))
}
