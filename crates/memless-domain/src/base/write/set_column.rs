use crate::scalar::Scalar;

pub(crate) fn set_column(columns: &mut Vec<(String, Scalar)>, name: &str, value: &Scalar) {
    if let Some(entry) = columns.iter_mut().find(|(key, _)| key == name) {
        entry.1 = value.clone();
        return;
    }
    columns.push((name.to_string(), value.clone()));
}
