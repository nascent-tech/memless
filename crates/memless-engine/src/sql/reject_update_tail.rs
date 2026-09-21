use super::outside::outside;
use memless_domain::QueryRefusal;

pub(crate) fn reject_update_tail(has_from: bool, has_returning: bool, has_or: bool) -> Result<(), QueryRefusal> {
    if has_from {
        return Err(outside("UPDATE with FROM"));
    }
    if has_returning {
        return Err(outside("RETURNING"));
    }
    if has_or {
        return Err(outside("UPDATE with a conflict modifier"));
    }
    Ok(())
}
