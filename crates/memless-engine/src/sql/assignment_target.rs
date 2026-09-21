use super::column_name::column_name;
use super::outside::outside;
use memless_domain::QueryRefusal;
use sqlparser::ast::AssignmentTarget;

pub(crate) fn target_column(target: &AssignmentTarget) -> Result<String, QueryRefusal> {
    match target {
        AssignmentTarget::ColumnName(name) => column_name(name),
        AssignmentTarget::Tuple(_) => Err(outside("tuple assignment")),
    }
}
