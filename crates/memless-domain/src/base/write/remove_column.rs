use crate::scalar::Scalar;

pub(crate) fn remove_column(columns: &mut Vec<(String, Scalar)>, name: &str) {
    columns.retain(|(key, _)| key != name);
}
