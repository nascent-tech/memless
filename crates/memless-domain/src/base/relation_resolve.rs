use std::collections::HashSet;

use super::relation_context::RelationContext;
use crate::relation::guessed_target;
use crate::Id;

pub(crate) fn relation_rows<'a>(
    ctx: &'a RelationContext,
    column: &str,
) -> Option<(String, &'a HashSet<&'a Id>)> {
    let target = guessed_target(column)?;
    let ids = ctx.index.get(target.as_str())?;
    Some((target, ids))
}
