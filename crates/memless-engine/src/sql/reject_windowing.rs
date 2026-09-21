use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::Select;

pub(crate) fn reject_windowing(select: &Select) -> Result<(), QueryRefusal> {
    if !select.named_window.is_empty() {
        return Err(outside("WINDOW"));
    }
    if select.qualify.is_some() {
        return Err(outside("QUALIFY"));
    }
    if select.connect_by.is_some() || select.value_table_mode.is_some() {
        return Err(outside("CONNECT BY"));
    }
    Ok(())
}
