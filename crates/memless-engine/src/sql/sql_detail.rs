use sqlparser::parser::ParserError;

pub(crate) fn sql_detail(error: ParserError) -> String {
    error.to_string().trim_start_matches("sql parser error: ").to_string()
}
