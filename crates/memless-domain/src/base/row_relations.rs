use super::column_relation::check_column_relation;
use super::index::IdIndex;
use super::relation_context::RelationContext;
use super::row::Row;
use super::table::Table;
use crate::refusal::{RowLabel, StructureRefusal};

pub(crate) fn check_row_relations(index: &IdIndex, table: &Table, row: &Row) -> Result<(), StructureRefusal> {
    let ctx = RelationContext { index, table: &table.name, row: RowLabel::Id(row.id.clone()) };
    row.columns.iter().try_for_each(|(column, value)| check_column_relation(&ctx, column, value))
}
