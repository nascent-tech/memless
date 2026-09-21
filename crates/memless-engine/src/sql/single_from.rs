use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::TableWithJoins;

pub(crate) fn single_from(from: &[TableWithJoins]) -> Result<&TableWithJoins, QueryRefusal> {
    if from.is_empty() {
        return Err(outside("SELECT without FROM"));
    }
    if from.len() > 1 {
        return Err(outside("comma join"));
    }
    Ok(&from[0])
}
