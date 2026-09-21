use std::fmt;

use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Text(String),
    Integer(i64),
}

impl Id {
    pub fn from_scalar(value: Scalar) -> Option<Id> {
        match value {
            Scalar::Text(text) => Some(Id::Text(text)),
            Scalar::Integer(number) => Some(Id::Integer(number)),
            Scalar::Decimal(_) | Scalar::Boolean(_) => None,
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::Text(text) => write!(formatter, "{}", Scalar::Text(text.clone())),
            Id::Integer(number) => write!(formatter, "{}", Scalar::Integer(*number)),
        }
    }
}
