use super::index::IdIndex;
use crate::refusal::RowLabel;

pub(crate) struct RelationContext<'a> {
    pub index: &'a IdIndex<'a>,
    pub table: &'a str,
    pub row: RowLabel,
}
