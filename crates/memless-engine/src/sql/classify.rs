use super::item_kind::ItemKind;
use super::lower_item::lower_item;
use memless_domain::QueryRefusal;
use sqlparser::ast::SelectItem;

pub(crate) fn classify(projection: &[SelectItem], has_join: bool) -> Result<Vec<ItemKind>, QueryRefusal> {
    projection.iter().map(|item| lower_item(item, has_join)).collect()
}
