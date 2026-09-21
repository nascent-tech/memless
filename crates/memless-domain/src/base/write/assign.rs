use super::assign_one::assign_one;
use crate::scalar::Scalar;

pub(crate) fn apply_assignments(
    columns: &[(String, Scalar)],
    assignments: &[(String, Option<Scalar>)],
) -> Vec<(String, Scalar)> {
    let mut result = columns.to_vec();
    for (name, value) in assignments {
        assign_one(&mut result, name, value);
    }
    result
}
