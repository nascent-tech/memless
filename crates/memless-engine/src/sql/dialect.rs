use sqlparser::dialect::Dialect;

#[derive(Debug)]
pub(crate) struct MemlessDialect;

impl Dialect for MemlessDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '"'
    }
}
