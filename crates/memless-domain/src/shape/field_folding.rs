use super::field_parts::{claim_column, column_name, field_scalar};
use super::field_site::{FieldSite, RowAcc};
use crate::document::{RawKey, RawNode};
use crate::refusal::StructureRefusal;

pub(crate) fn fold_field(
    site: &FieldSite,
    mut acc: RowAcc,
    key: &RawKey,
    value: &RawNode,
) -> Result<RowAcc, StructureRefusal> {
    let column = column_name(site, key)?;
    claim_column(site, &mut acc.0, &column)?;
    acc.1.extend(field_scalar(site, &column, value)?.map(|scalar| (column, scalar)));
    Ok(acc)
}
