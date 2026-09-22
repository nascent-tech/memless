use super::outside::outside;
use memless_domain::QueryRefusal;

pub(crate) fn reject_transaction_options(present: bool) -> Result<(), QueryRefusal> {
    if present {
        return Err(outside("transaction options"));
    }
    Ok(())
}
