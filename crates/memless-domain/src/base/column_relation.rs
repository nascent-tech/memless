use super::faults::broken_relation;
use super::relation_context::RelationContext;
use super::relation_lookup::contains_id;
use super::relation_resolve::relation_rows;
use crate::refusal::StructureRefusal;
use crate::scalar::Scalar;

pub(crate) fn check_column_relation(
    ctx: &RelationContext,
    column: &str,
    value: &Scalar,
) -> Result<(), StructureRefusal> {
    match relation_rows(ctx, column) {
        Some((target, ids)) if !contains_id(ids, value) => Err(broken_relation(ctx, column, &target, value)),
        _ => Ok(()),
    }
}
