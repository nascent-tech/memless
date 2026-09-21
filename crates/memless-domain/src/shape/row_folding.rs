use super::field_folding::fold_field;
use super::field_site::{FieldSite, RowAcc, ShapedRow};
use crate::document::{RawKey, RawNode};
use crate::refusal::StructureRefusal;

pub(crate) fn fold_row(site: &FieldSite, entries: &[(RawKey, RawNode)]) -> Result<ShapedRow, StructureRefusal> {
    let start: RowAcc = (Vec::new(), Vec::new());
    let (_, fields) = entries
        .iter()
        .try_fold(start, |acc, (key, value)| fold_field(site, acc, key, value))?;
    Ok(fields)
}
