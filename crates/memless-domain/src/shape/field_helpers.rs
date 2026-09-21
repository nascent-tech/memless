use super::field_site::FieldSite;
use crate::document::{RawKey, RawNode};
use crate::refusal::StructureRefusal;
use crate::refusal::StructureRefusal::{DuplicateColumnKey, NestedValue, NonTextKey};
use crate::scalar::Scalar;
use crate::Id;

pub(crate) fn bare_id(key: &RawKey, value: &RawNode) -> Option<Id> {
    let RawKey::Text(name) = key else { return None };
    let RawNode::Scalar(raw) = value else { return None };
    if name.as_str() != "id" {
        return None;
    }
    Id::from_scalar(Scalar::guess(raw))
}

pub(crate) fn non_text_column(site: &FieldSite, rendered: &str) -> StructureRefusal {
    NonTextKey { rendered: rendered.to_string(), table: Some(site.table.to_string()), position: site.position }
}

pub(crate) fn duplicate_column(site: &FieldSite, column: &str) -> StructureRefusal {
    DuplicateColumnKey { table: site.table.to_string(), row: site.label.clone(), column: column.to_string() }
}

pub(crate) fn nested_value(site: &FieldSite, column: &str) -> StructureRefusal {
    NestedValue { table: site.table.to_string(), row: site.label.clone(), column: column.to_string() }
}
