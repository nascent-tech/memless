use super::bare_options::bare_options;
use sqlparser::ast::SelectItem;

pub(crate) fn plain_wildcard(item: &SelectItem) -> bool {
    match item {
        SelectItem::Wildcard(options) => bare_options(options),
        _ => false,
    }
}
