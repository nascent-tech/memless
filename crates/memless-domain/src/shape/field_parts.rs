use super::field_helpers::{bare_id, duplicate_column, nested_value, non_text_column};
use super::field_site::FieldSite;
use crate::document::{RawKey, RawNode};
use crate::refusal::{RowLabel, StructureRefusal};
use crate::scalar::Scalar;

pub(crate) fn row_label(entries: &[(RawKey, RawNode)], position: usize) -> RowLabel {
    let found = entries.iter().find_map(|(key, value)| bare_id(key, value));
    found.map(RowLabel::Id).unwrap_or(RowLabel::Position(position))
}

pub(crate) fn column_name(site: &FieldSite, key: &RawKey) -> Result<String, StructureRefusal> {
    match key {
        RawKey::Text(name) => Ok(name.clone()),
        RawKey::NonText { rendered } => Err(non_text_column(site, rendered)),
    }
}

pub(crate) fn claim_column(site: &FieldSite, seen: &mut Vec<String>, column: &str) -> Result<(), StructureRefusal> {
    if seen.iter().any(|name| name == column) {
        return Err(duplicate_column(site, column));
    }
    seen.push(column.to_string());
    Ok(())
}

pub(crate) fn field_scalar(
    site: &FieldSite,
    column: &str,
    value: &RawNode,
) -> Result<Option<Scalar>, StructureRefusal> {
    match value {
        RawNode::Scalar(raw) => Ok(Some(Scalar::guess(raw))),
        RawNode::Null => Ok(None),
        _ => Err(nested_value(site, column)),
    }
}
