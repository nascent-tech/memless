use super::select::Select;
use super::write::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select(Select),
    Write(Write),
}
