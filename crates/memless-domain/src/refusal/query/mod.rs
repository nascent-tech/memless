use std::fmt;

use super::RowLabel;

mod execution;
mod name;
mod syntax;

use execution::execution_message;
use name::name_message;
use syntax::syntax_message;

#[derive(Debug, Clone, PartialEq)]
pub enum QueryRefusal {
    InvalidSql { detail: String },
    OutsideSubset { construct: String },
    UnknownTable { table: String },
    UnknownColumn { table: String, column: String },
    JoinNotRelation { table: String, column: String, target: String },
    SumNotNumber { table: String, column: String, row: RowLabel },
    SumOverflow { table: String, column: String },
}

impl fmt::Display for QueryRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = syntax_message(self)
            .or_else(|| name_message(self))
            .or_else(|| execution_message(self));
        write!(formatter, "{}", message.unwrap_or_default())
    }
}
