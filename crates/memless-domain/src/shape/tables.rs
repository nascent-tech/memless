use crate::document::{RawDocument, RawKey, RawNode};
use crate::refusal::StructureRefusal;
use crate::refusal::StructureRefusal::{DuplicateTableKey, NoTableDeclared, NonTextKey};

pub(crate) fn table_entries(document: &RawDocument) -> Result<&[(RawKey, RawNode)], StructureRefusal> {
    match &document.root {
        RawNode::Mapping(entries) if !entries.is_empty() => Ok(entries),
        _ => Err(NoTableDeclared { source: document.source.clone() }),
    }
}

pub(crate) fn table_name(key: &RawKey, position: usize) -> Result<String, StructureRefusal> {
    match key {
        RawKey::Text(name) => Ok(name.clone()),
        RawKey::NonText { rendered } => Err(NonTextKey { rendered: rendered.clone(), table: None, position }),
    }
}

pub(crate) fn ensure_new_table(seen: &mut Vec<String>, name: &str) -> Result<(), StructureRefusal> {
    if seen.iter().any(|existing| existing == name) {
        return Err(DuplicateTableKey { table: name.to_string() });
    }
    seen.push(name.to_string());
    Ok(())
}
