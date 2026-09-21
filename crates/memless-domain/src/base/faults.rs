use super::relation_context::RelationContext;
use crate::refusal::StructureRefusal;
use crate::refusal::StructureRefusal::{BrokenRelation, DuplicateId, IdNotTextOrInteger, MissingId};
use crate::scalar::Scalar;
use crate::Id;

pub(crate) fn missing_id(table: &str, position: usize) -> StructureRefusal {
    MissingId { table: table.to_string(), position }
}

pub(crate) fn bad_id(table: &str, position: usize, value: Scalar) -> StructureRefusal {
    IdNotTextOrInteger { table: table.to_string(), position, value }
}

pub(crate) fn duplicate_id(table: &str, id: Id, first: usize, second: usize) -> StructureRefusal {
    DuplicateId { table: table.to_string(), id, positions: (first, second) }
}

pub(crate) fn broken_relation(ctx: &RelationContext, column: &str, target: &str, value: &Scalar) -> StructureRefusal {
    BrokenRelation {
        table: ctx.table.to_string(),
        row: ctx.row.clone(),
        column: column.to_string(),
        target_table: target.to_string(),
        value: value.clone(),
    }
}
