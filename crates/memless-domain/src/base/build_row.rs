use super::row::Row;
use super::row_id::row_id;
use crate::refusal::StructureRefusal;
use crate::scalar::Scalar;

pub(crate) fn build_row(table: &str, position: usize, fields: &[(String, Scalar)]) -> Result<Row, StructureRefusal> {
    let id = row_id(table, position, fields)?;
    Ok(Row { id, columns: fields.to_vec() })
}
