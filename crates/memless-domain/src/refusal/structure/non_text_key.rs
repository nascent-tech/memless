use super::StructureRefusal;
use super::StructureRefusal::NonTextKey;

pub(crate) fn non_text_key_message(refusal: &StructureRefusal) -> Option<String> {
    let NonTextKey { rendered, table, position } = refusal else {
        return None;
    };
    let scope = match table {
        Some(name) => format!(" in table {name:?}"),
        None => String::new(),
    };
    Some(format!("non-text key {rendered} at position {position}{scope}"))
}
