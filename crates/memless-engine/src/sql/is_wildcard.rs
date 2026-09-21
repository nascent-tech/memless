use super::plain_wildcard::plain_wildcard;
use sqlparser::ast::SelectItem;

pub(crate) fn is_wildcard(projection: &[SelectItem]) -> bool {
    projection.len() == 1 && plain_wildcard(&projection[0])
}
