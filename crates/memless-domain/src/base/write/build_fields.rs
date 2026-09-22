use crate::scalar::Scalar;

pub(crate) fn build_fields(columns: &[String], values: &[Option<Scalar>]) -> Vec<(String, Scalar)> {
    columns
        .iter()
        .zip(values)
        .filter_map(|(name, value)| value.clone().map(|scalar| (name.clone(), scalar)))
        .collect()
}
