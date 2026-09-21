#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemlessStatus {
    Ok,
    Refused,
    InvalidArgument,
    Internal,
}

pub type MemlessHandle = u64;
