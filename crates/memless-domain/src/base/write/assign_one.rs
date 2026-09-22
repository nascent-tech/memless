use super::remove_column::remove_column;
use super::set_column::set_column;
use crate::scalar::Scalar;

pub(crate) fn assign_one(columns: &mut Vec<(String, Scalar)>, name: &str, value: &Option<Scalar>) {
    match value {
        Some(scalar) => set_column(columns, name, scalar),
        None => remove_column(columns, name),
    }
}
