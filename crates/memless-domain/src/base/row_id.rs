use super::faults::{bad_id, missing_id};
use crate::refusal::StructureRefusal;
use crate::scalar::Scalar;
use crate::Id;

pub(crate) fn row_id(table: &str, position: usize, fields: &[(String, Scalar)]) -> Result<Id, StructureRefusal> {
    let found = fields.iter().find(|entry| entry.0.as_str() == "id");
    let Some((_, scalar)) = found else {
        return Err(missing_id(table, position));
    };
    Id::from_scalar(scalar.clone()).ok_or_else(|| bad_id(table, position, scalar.clone()))
}
