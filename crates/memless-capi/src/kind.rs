#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemlessKind {
    Absent,
    Text,
    Integer,
    Decimal,
    Boolean,
}
