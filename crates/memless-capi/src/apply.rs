use super::kind::MemlessKind;
use super::payload::Payload;
use super::resolved::Resolved;
use super::write_boolean::write_boolean;
use super::write_decimal::write_decimal;
use super::write_integer::write_integer;
use super::write_text::write_text;

/// # Safety
/// The pointers in `payload` must each be null or writable for the call.
pub(crate) unsafe fn apply(resolved: Resolved, payload: &Payload) -> MemlessKind {
    match resolved {
        Resolved::Absent => MemlessKind::Absent,
        Resolved::Text(pointer) => write_text(payload, pointer),
        Resolved::Integer(number) => write_integer(payload, number),
        Resolved::Decimal(number) => write_decimal(payload, number),
        Resolved::Boolean(flag) => write_boolean(payload, flag),
    }
}
