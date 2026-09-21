use crate::query::Update;

pub(crate) fn assignment_targets(spec: &Update) -> Vec<String> {
    spec.assignments.iter().map(|(name, _)| name.clone()).collect()
}
