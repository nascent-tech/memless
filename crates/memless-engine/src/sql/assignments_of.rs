use super::one_assignment::one_assignment;
use memless_domain::Scalar;
use memless_domain::QueryRefusal;
use sqlparser::ast::Assignment;

pub(crate) fn assignments_of(assignments: &[Assignment]) -> Result<Vec<(String, Option<Scalar>)>, QueryRefusal> {
    assignments.iter().map(one_assignment).collect()
}
