use super::QueryRefusal;
use super::QueryRefusal::{InvalidSql, OutsideSubset};

pub(crate) fn syntax_message(refusal: &QueryRefusal) -> Option<String> {
    match refusal {
        InvalidSql { detail } => Some(format!("invalid SQL: {detail}")),
        OutsideSubset { construct } => Some(format!("{construct} is outside the supported SQL subset")),
        _ => None,
    }
}
