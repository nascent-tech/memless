use sqlparser::ast::Ident;

pub(crate) fn ident_name(ident: &Ident) -> String {
    ident.value.clone()
}
